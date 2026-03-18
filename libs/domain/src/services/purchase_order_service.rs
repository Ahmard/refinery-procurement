//! Purchase Order Service - Business logic for procurement order management
//!
//! This service handles all mutation operations for purchase orders in the procurement system.
//! Purchase orders represent requests to suppliers for refinery equipment and materials.
//!
//! # Responsibilities
//! - Creating draft purchase orders
//! - Adding items to orders
//! - Submitting orders for approval
//! - Canceling orders
//! - Managing order lifecycle
//! - Enforcing idempotency
//!
//! # Business Rules
//! - Each purchase order must have exactly one supplier
//! - Only DRAFT orders can have items added or be submitted
//! - Submitted orders cannot be modified
//! - Idempotency keys prevent duplicate orders
//! - Price and lead time are snapshotted at submission

use crate::dto::purchase_order_dto::{
    IdempotencyKey, PurchaseOrderCreateRequest, PurchaseOrderItemRequest,
    PurchaseOrderItemResponse, PurchaseOrderResponse, StatusHistoryEntry,
};
use crate::dto::AuditLogCreateRequest;
use crate::enums::AppEvent;
use crate::event::Event;
use crate::repositories::{
    CatalogItemRepository, PurchaseOrderItemRepository, PurchaseOrderRepository,
    PurchaseOrderStatusHistoryRepository, SupplierRepository,
};
use crate::services::AuditLogService;
use bigdecimal::BigDecimal;
use chrono::Utc;
use database::enums::purchase_order_status::PurchaseOrderStatus;
use database::models::purchase_order::{PurchaseOrder, PurchaseOrderInsertable};
use database::models::purchase_order_item::PurchaseOrderItemInsertable;
use database::models::purchase_order_status_history::PurchaseOrderStatusHistoryInsertable;
use foxtive::helpers::run_async;
use foxtive::prelude::AppResult;
use foxtive::{conflict, invalid, not_found};
use serde_json::json;
use tracing::{debug, info};
use uuid::Uuid;

/// PurchaseOrderService handles all purchase order mutation operations
///
/// This service orchestrates repository calls while enforcing business rules
/// for the procurement workflow.
pub struct PurchaseOrderService;

impl PurchaseOrderService {
    /// Create a new draft purchase order
    ///
    /// # Business Rules
    /// - Idempotency key prevents duplicates
    /// - Supplier must exist
    /// - Initial status is DRAFT
    /// - PO number is auto-generated
    ///
    /// # Arguments
    /// * `request` - Purchase order creation details
    /// * `idempotency_key` - Optional key to prevent duplicates
    /// * `created_by` - ID of the user creating the order
    ///
    /// # Returns
    /// The created purchase order with PO number
    pub fn create_draft_order(
        request: PurchaseOrderCreateRequest,
        idempotency_key: IdempotencyKey,
        created_by: Uuid,
    ) -> AppResult<PurchaseOrder> {
        let idempotency_key = idempotency_key.0;
        let supplier_id = request.supplier_id;
        info!(
            supplier_id = %supplier_id,
            idempotency_key = idempotency_key,
            "Creating draft purchase order"
        );
        debug!("Validating purchase order creation parameters");

        // Check idempotency - return existing order if key already used
        if let Some(existing) = PurchaseOrderRepository::find_by_idempotency_key(&idempotency_key)?
        {
            info!(order_id = %existing.id, "Returning existing order for idempotency key");
            return Ok(existing);
        }

        // Generate unique PO number
        let po_number = Self::generate_po_number()?;

        // Build insertable purchase order
        let new_order = PurchaseOrderInsertable {
            po_number: Some(po_number),
            supplier_id,
            created_by,
            requestor: request.requestor,
            cost_center: request.cost_center,
            payment_terms: request.payment_terms,
            needed_by_date: request.needed_by_date,
            status: PurchaseOrderStatus::Draft,
            idempotency_key: Some(idempotency_key),
            submitted_at: None,
            total_cost: BigDecimal::from(0),
        };

        // Create order via repository
        let created_order = PurchaseOrderRepository::create(new_order)?;

        // Record audit log
        let _ = AuditLogService::record_activity(AuditLogCreateRequest {
            user_id: created_by,
            action: "create".to_string(),
            target_entity: "purchase_order".to_string(),
            target_id: created_order.id.to_string(),
            changes: Some(json!({
                "status": "DRAFT",
                "po_number": created_order.po_number,
            })),
        });

        // Post-operation hook
        info!(
            order_id = %created_order.id,
            po_number = %created_order.po_number.as_ref().unwrap(),
            "Purchase order created successfully"
        );
        Self::emit_event(AppEvent::PurchaseOrderCreated, &created_order)?;

        Ok(created_order)
    }

    /// Add an item to a draft purchase order
    ///
    /// # Business Rules
    /// - Order must be in DRAFT status
    /// - All items must be from the same supplier
    /// - Item must exist in catalog
    /// - Quantity must be positive
    /// - Price and lead time are captured at time of adding
    ///
    /// # Arguments
    /// * `order_id` - UUID of the purchase order
    /// * `request` - Item details to add
    /// * `created_by` - ID of the user adding the item
    ///
    /// # Returns
    /// The updated purchase order
    pub fn add_item_to_order(
        order_id: Uuid,
        request: PurchaseOrderItemRequest,
        created_by: Uuid,
    ) -> AppResult<PurchaseOrder> {
        info!(
            order_id = %order_id,
            item_id = %request.item_id,
            quantity = request.quantity,
            "Adding item to purchase order"
        );
        debug!("Validating order and item before adding");

        // Fetch the order
        let order = PurchaseOrderRepository::find(order_id)?;

        // Business rule: Order must be in DRAFT status
        if order.status != PurchaseOrderStatus::Draft {
            return Err(invalid!(
                "Cannot add items to order in {} status. Only DRAFT orders can be modified",
                order.status
            ));
        }

        // Find catalog item by secondary ID
        let catalog_item = CatalogItemRepository::find_by_secondary_id(&request.item_id)?
            .ok_or_else(|| not_found!("Catalog item '{}' not found", request.item_id))?;

        // Business rule: All items must be from the same supplier
        // Check existing items in the order
        let existing_items = PurchaseOrderItemRepository::list_by_order(order_id)?;
        if !existing_items.is_empty() {
            // Get the supplier for this order (from first item's catalog)
            let first_item_catalog =
                CatalogItemRepository::find(existing_items[0].catalog_item_id)?;
            if first_item_catalog.supplier_id != catalog_item.supplier_id {
                return Err(conflict!(
                    "All items in a purchase order must be from the same supplier. \
                     Expected supplier ID {}, but item '{}' belongs to supplier ID {}",
                    first_item_catalog.supplier_id,
                    request.item_id,
                    catalog_item.supplier_id
                ));
            }
        }

        // Verify order's supplier matches item's supplier
        if order.supplier_id != catalog_item.supplier_id {
            return Err(conflict!(
                "Item supplier mismatch. Order supplier: {}, Item supplier: {}",
                order.supplier_id,
                catalog_item.supplier_id
            ));
        }

        // Calculate total price for this line item
        let unit_price = catalog_item.price_usd.clone();
        let _total_price = &unit_price * BigDecimal::from(request.quantity as i64);

        // Add item via repository
        PurchaseOrderItemRepository::add_item(PurchaseOrderItemInsertable {
            purchase_order_id: order_id,
            catalog_item_id: catalog_item.id,
            quantity: BigDecimal::from(request.quantity),
            snapshot_price: unit_price,
            snapshot_lead_time: catalog_item.lead_time_days,
            created_by,
        })?;

        // Record audit log
        let _ = AuditLogService::record_activity(AuditLogCreateRequest {
            user_id: created_by,
            action: "add_item".to_string(),
            target_entity: "purchase_order".to_string(),
            target_id: order_id.to_string(),
            changes: Some(json!({
                "item_id": catalog_item.secondary_id,
                "quantity": request.quantity,
            })),
        });

        // Return updated order
        let updated_order = Self::recompute_price(order_id)?;

        info!(
            order_id = %updated_order.id,
            item_id = %catalog_item.id,
            "Item added to purchase order successfully"
        );

        Ok(updated_order)
    }

    /// Submit a draft purchase order for approval
    ///
    /// # Business Rules
    /// - Order must be in DRAFT status
    /// - Order must have at least one item
    /// - Status changes to SUBMITTED
    /// - submitted_at timestamp is set
    /// - Price and lead time are snapshotted (already done when adding items)
    /// - Status history is recorded
    ///
    /// # Arguments
    /// * `order_id` - UUID of the purchase order to submit
    /// * `submitted_by` - ID of the user submitting the order
    ///
    /// # Returns
    /// The submitted purchase order
    pub fn submit_order(order_id: Uuid, submitted_by: Uuid) -> AppResult<PurchaseOrder> {
        info!(order_id = %order_id, "Submitting purchase order");
        debug!("Validating order can be submitted");

        // Fetch the order
        let mut order = PurchaseOrderRepository::find(order_id)?;

        // Business rule: Order must be in DRAFT status
        if order.status != PurchaseOrderStatus::Draft {
            return Err(invalid!(
                "Cannot submit order in {} status. Only DRAFT orders can be submitted",
                order.status
            ));
        }

        // Business rule: Order must have at least one item
        let items = PurchaseOrderItemRepository::list_by_order(order_id)?;
        if items.is_empty() {
            return Err(invalid!(
                "Cannot submit purchase order without items. Add at least one item first"
            ));
        }

        // Update order status and timestamp
        order.status = PurchaseOrderStatus::Submitted;
        order.submitted_at = Some(Utc::now().naive_utc());

        // Persist the update
        let submitted_order = PurchaseOrderRepository::update(order)?;

        // Record status change in history
        Self::record_status_change(
            submitted_order.id,
            PurchaseOrderStatus::Submitted,
            submitted_by,
        )?;

        // Record audit log
        let _ = AuditLogService::record_activity(AuditLogCreateRequest {
            user_id: submitted_by,
            action: "submit".to_string(),
            target_entity: "purchase_order".to_string(),
            target_id: submitted_order.id.to_string(),
            changes: Some(json!({
                "status": "SUBMITTED",
            })),
        });

        info!(
            order_id = %submitted_order.id,
            po_number = %submitted_order.po_number.as_ref().unwrap(),
            "Purchase order submitted successfully"
        );
        Self::emit_event(AppEvent::PurchaseOrderSubmitted, &submitted_order)?;

        Ok(submitted_order)
    }

    /// Cancel a purchase order
    ///
    /// # Business Rules
    /// - Order cannot be cancelled if already APPROVED
    /// - Status changes to CANCELLED
    /// - Status history is recorded
    ///
    /// # Arguments
    /// * `order_id` - UUID of the purchase order to cancel
    /// * `cancelled_by` - ID of the user cancelling the order
    ///
    /// # Returns
    /// The cancelled purchase order
    pub fn cancel_order(order_id: Uuid, cancelled_by: Uuid) -> AppResult<PurchaseOrder> {
        info!(order_id = %order_id, "Cancelling purchase order");
        debug!("Validating order can be cancelled");

        // Fetch the order
        let mut order = PurchaseOrderRepository::find(order_id)?;

        // Business rule: Cannot cancel approved orders
        if order.status == PurchaseOrderStatus::Approved {
            return Err(invalid!("Cannot cancel order that has been APPROVED"));
        }

        // Update status
        order.status = PurchaseOrderStatus::Cancelled;

        // Persist the update
        let cancelled_order = PurchaseOrderRepository::update(order)?;

        // Record status change in history
        Self::record_status_change(
            cancelled_order.id,
            PurchaseOrderStatus::Cancelled,
            cancelled_by,
        )?;

        // Record audit log
        let _ = AuditLogService::record_activity(AuditLogCreateRequest {
            user_id: cancelled_by,
            action: "cancel".to_string(),
            target_entity: "purchase_order".to_string(),
            target_id: cancelled_order.id.to_string(),
            changes: Some(json!({
                "status": "CANCELLED",
            })),
        });

        info!(
            order_id = %cancelled_order.id,
            "Purchase order cancelled successfully"
        );
        Self::emit_event(AppEvent::PurchaseOrderCancelled, &cancelled_order)?;

        Ok(cancelled_order)
    }

    /// Get complete purchase order details with items and history
    ///
    /// # Arguments
    /// * `order_id` - UUID of the purchase order
    ///
    /// # Returns
    /// Aggregated response with order, items, and status history
    pub fn get_order_details(order_id: Uuid) -> AppResult<PurchaseOrderResponse> {
        info!(order_id = %order_id, "Getting purchase order details");
        debug!("Fetching order, items, and history");

        // Fetch the order
        let order = PurchaseOrderRepository::find(order_id)?;

        // Fetch all line items
        let items = PurchaseOrderItemRepository::list_by_order(order_id)?;

        // Build item responses with catalog item names
        let item_responses: Vec<PurchaseOrderItemResponse> = items
            .into_iter()
            .map(|item| {
                // Get catalog item name for display
                let catalog_item = CatalogItemRepository::find(item.catalog_item_id)
                    .map(|ci| ci.name)
                    .unwrap_or_else(|_| "Unknown Item".to_string());

                let total_price = &item.snapshot_price * item.quantity.clone();

                PurchaseOrderItemResponse {
                    id: item.id,
                    catalog_item_id: item.catalog_item_id.to_string(), // This should be secondary_id ideally
                    item_name: catalog_item,
                    quantity: item.quantity,
                    unit_price: item.snapshot_price,
                    total_price,
                    snapshot_lead_time: item.snapshot_lead_time,
                }
            })
            .collect();

        // Calculate total amount
        let total_amount = item_responses
            .iter()
            .fold(BigDecimal::from(0), |acc, item| acc + &item.total_price);

        // Fetch supplier name
        let supplier = SupplierRepository::find(order.supplier_id)?;

        // Fetch status history
        let status_history = PurchaseOrderStatusHistoryRepository::find_by_order_id(order_id)?;
        let history_entries: Vec<StatusHistoryEntry> = status_history
            .into_iter()
            .map(|h| StatusHistoryEntry {
                status: h.status,
                created_at: h.created_at,
                created_by: h.created_by,
            })
            .collect();

        Ok(PurchaseOrderResponse {
            id: order.id,
            po_number: order.po_number.unwrap_or_default(),
            supplier_id: order.supplier_id,
            supplier_name: supplier.name,
            status: order.status,
            items: item_responses,
            total_amount,
            requestor: order.requestor,
            cost_center: order.cost_center,
            payment_terms: order.payment_terms,
            needed_by_date: order.needed_by_date,
            created_at: order.created_at,
            updated_at: order.updated_at,
            submitted_at: order.submitted_at,
            status_history: history_entries,
        })
    }

    /// Generate a unique purchase order number
    ///
    /// Strategy: PO-YYYYMMDD-XXXXX (date + sequence)
    ///
    /// # Returns
    /// A unique PO number string
    fn generate_po_number() -> AppResult<String> {
        let now = Utc::now();
        let date_str = now.format("%Y%m%d").to_string();

        // Simple approach: use UUID and take first 5 chars of hyphenated format
        let unique_id = Uuid::new_v4();
        let seq = unique_id.hyphenated().to_string()[..5].to_uppercase();

        Ok(format!("PO-{}-{}", date_str, seq))
    }

    /// Record a status change in the history table
    fn record_status_change(
        order_id: Uuid,
        new_status: PurchaseOrderStatus,
        changed_by: Uuid,
    ) -> AppResult<()> {
        debug!(
            order_id = %order_id,
            ?new_status,
            "Recording status change in history"
        );

        let history_entry = PurchaseOrderStatusHistoryInsertable {
            purchase_order_id: order_id,
            status: new_status,
            created_by: changed_by,
        };

        PurchaseOrderStatusHistoryRepository::create(history_entry)?;
        Ok(())
    }

    /// Calculate and update total price for an order
    pub fn recompute_price(id: Uuid) -> AppResult<PurchaseOrder> {
        let mut order = PurchaseOrderRepository::find(id)?;
        order.total_cost = PurchaseOrderItemRepository::sum_cost(id)?;
        PurchaseOrderRepository::update(order)
    }

    /// Emit an event for external systems (audit logs, event streams, etc.)
    fn emit_event(event_type: AppEvent, order: &PurchaseOrder) -> AppResult<()> {
        run_async(Event::emit(event_type, order))?;
        debug!(order_id = %order.id, "[{event_type}] Event emitted");
        Ok(())
    }
}
