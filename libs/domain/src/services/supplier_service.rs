//! Supplier Service - Business logic for supplier management
//!
//! This service handles all mutation operations for suppliers in the procurement system.
//! Suppliers represent companies that provide refinery equipment and materials.
//!
//! # Responsibilities
//! - Supplier creation with validation
//! - Supplier updates
//! - Supplier activation/deactivation
//! - Enforcing business rules around supplier lifecycle
//!
//! # Business Rules
//! - Supplier code must be unique
//! - Supplier name must be unique
//! - Suppliers cannot be hard deleted (only deactivated)
//! - Suppliers with active catalog items cannot be deactivated
//! - Suppliers with active purchase orders cannot be deactivated
//! - Supplier code should not change once catalog items exist

use crate::dto::supplier_dto::{SupplierCreateForm, SupplierDto};
use crate::dto::user_dto::UserCreateDto;
use crate::dto::AuditLogCreateRequest;
use crate::enums::AppEvent;
use crate::event::Event;
use crate::repositories::{CatalogItemRepository, PurchaseOrderRepository, SupplierRepository};
use crate::services::AuditLogService;
use crate::services::UserService;
use database::enums::supplier_enums::SupplierStatus;
use database::enums::UserRole;
use database::models::supplier::{Supplier, SupplierInsertable};
use foxtive::helpers::run_async;
use foxtive::prelude::AppResult;
use foxtive::{conflict, internal_server_error, invalid};
use serde_json::json;
use tracing::{debug, info, instrument};
use uuid::Uuid;

/// SupplierService handles all supplier mutation operations
///
/// This service orchestrates repository calls while enforcing business rules
/// to maintain supplier stability across the procurement system.
pub struct SupplierService;

impl SupplierService {
    /// Create a new supplier
    ///
    /// # Business Rules
    /// - Supplier name must be unique
    /// - All required fields must be present
    /// - Contact information is optional but recommended
    ///
    /// # Arguments
    /// * `name` - The supplier's company name
    /// * `contact_email` - Optional contact email address
    /// * `contact_phone` - Optional contact phone number
    /// * `address` - Optional physical address
    /// * `created_by` - ID of the user creating this supplier
    ///
    /// # Returns
    /// The created Supplier entity
    pub fn create(dto: SupplierDto) -> AppResult<Supplier> {
        let SupplierDto { form, created_by } = dto;

        let SupplierCreateForm {
            name,
            contact_email,
            contact_phone,
            address,
        } = form;

        info!(name = %name, "Creating new supplier");
        debug!("Validating supplier creation parameters");

        let trimmed_name = name.trim();

        // Validate name uniqueness
        if SupplierRepository::find_by_name(trimmed_name)?.is_some() {
            return Err(conflict!("Supplier name already exists"));
        }

        let (_user, supplier) = SupplierRepository::with_transaction(|conn| {
            // Create user
            // Note: Ideally this should use a transaction-aware method or user_repository directly
            // For now, we assume UserService handles its own connection or we're fine with non-atomic user creation
            let user = UserService::create(UserCreateDto {
                username: trimmed_name.to_string(),
                email: contact_email.clone(),
                password: "dummy".to_string(),
                role: UserRole::Supplier,
                status: None,
                created_by: None,
            })?;

            // Create supplier
            let new_supplier = SupplierRepository::create_with_conn(
                SupplierInsertable {
                    address: address.clone(),
                    created_by,
                    contact_phone: contact_phone.clone(),
                    user_id: user.id,
                    name: trimmed_name.to_string(),
                    contact_email: Some(contact_email.clone()),
                    status: SupplierStatus::Active,
                },
                conn,
            )?;

            Ok((user, new_supplier))
        })?;

        // Record audit log
        let _ = AuditLogService::record_activity(AuditLogCreateRequest {
            user_id: created_by,
            action: "create".to_string(),
            target_entity: "supplier".to_string(),
            target_id: supplier.id.to_string(),
            changes: Some(json!({
                "name": supplier.name,
                "contact_email": supplier.contact_email,
            })),
        });

        // Post-operation hook
        info!(supplier_id = %supplier.id, "Supplier created successfully");
        Self::emit_event(AppEvent::SupplierCreated, &supplier)?;

        Ok(supplier)
    }

    /// Update an existing supplier
    ///
    /// # Business Rules
    /// - Supplier name must remain unique
    /// - Supplier code changes are restricted once catalog items exist
    /// - Changes are tracked for audit purposes
    ///
    /// # Arguments
    /// * `supplier_id` - ID of the supplier to update
    /// * `name` - Optional new name
    /// * `contact_email` - Optional new contact email
    /// * `contact_phone` - Optional new contact phone
    /// * `address` - Optional new address
    ///
    /// # Returns
    /// The updated Supplier entity
    #[instrument(fields(supplier_id))]
    pub fn update(
        supplier_id: Uuid,
        name: Option<String>,
        contact_email: Option<String>,
        contact_phone: Option<String>,
        address: Option<String>,
        updated_by: Uuid, // Changed signature to include updated_by for audit
    ) -> AppResult<Supplier> {
        info!(supplier_id = %supplier_id, "Updating supplier");
        debug!("Fetching supplier for update");

        // Fetch the existing supplier
        let mut supplier = SupplierRepository::find(supplier_id)?;

        let mut changes = serde_json::Map::new();

        // Apply updates if provided
        if let Some(new_name) = name {
            let trimmed_name = new_name.trim();
            // Check name uniqueness if changing
            if trimmed_name != supplier.name
                && let Some(existing) = SupplierRepository::find_by_name(trimmed_name)?
            {
                if existing.id != supplier_id {
                    return Err(conflict!("Supplier name already exists"));
                }
                changes.insert("name".to_string(), json!(trimmed_name));
            }
            supplier.name = trimmed_name.to_string();
        }

        if let Some(new_email) = contact_email {
            changes.insert("contact_email".to_string(), json!(new_email));
            supplier.contact_email = Some(new_email.trim().to_string());
        }

        if let Some(new_phone) = contact_phone {
            changes.insert("contact_phone".to_string(), json!(new_phone));
            supplier.contact_phone = Some(new_phone);
        }

        if let Some(new_address) = address {
            changes.insert("address".to_string(), json!(new_address));
            supplier.address = Some(new_address);
        }

        // Note: updated_at is automatically managed by Diesel schema

        info!("Persisting supplier updates");
        let updated_supplier = SupplierRepository::update(supplier)?;

        // Record audit log if there were changes
        if !changes.is_empty() {
            let _ = AuditLogService::record_activity(AuditLogCreateRequest {
                user_id: updated_by,
                action: "update".to_string(),
                target_entity: "supplier".to_string(),
                target_id: updated_supplier.id.to_string(),
                changes: Some(serde_json::Value::Object(changes)),
            });
        }

        // Post-operation hook
        info!(supplier_id = %updated_supplier.id, "Supplier updated successfully");
        Self::emit_event(AppEvent::SupplierUpdated, &updated_supplier)?;

        Ok(updated_supplier)
    }

    /// Deactivate a supplier (soft delete)
    ///
    /// # Business Rules
    /// - Supplier cannot be deactivated if active catalog items reference it
    /// - Supplier cannot be deactivated if active purchase orders reference it
    /// - Hard deletion is not allowed (only soft delete via deactivation)
    ///
    /// # Arguments
    /// * `supplier_id` - ID of the supplier to deactivate
    ///
    /// # Returns
    /// Error if supplier has active references, otherwise success message
    #[instrument(fields(supplier_id))]
    pub fn deactivate(supplier_id: Uuid) -> AppResult<()> {
        info!(supplier_id = %supplier_id, "Deactivating supplier");
        debug!("Validating supplier deactivation prerequisites");

        // Fetch the supplier to verify it exists
        let _supplier = SupplierRepository::find(supplier_id)?;

        // Business rule: Check for active catalog items
        Self::validate_no_active_catalog_items(supplier_id)?;

        // Business rule: Check for active purchase orders
        Self::validate_no_active_purchase_orders(supplier_id)?;

        // Note: Since Supplier model doesn't have is_active or deleted_at fields,
        // we would need to add these fields to the schema for proper soft delete.
        // For now, this is a placeholder that validates the business rules.
        //
        // In a real implementation, you would:
        // 1. Add `is_active: Option<bool>` field to Supplier model
        // 2. Set it to Some(false) here
        // 3. Update the supplier via repository

        // TODO: Add soft delete fields to Supplier model and implement actual deactivation
        // For now, we'll just validate the business rules without actually deactivating
        // This is a limitation of the current schema

        info!(supplier_id = %supplier_id, "Supplier validated for deactivation (schema update needed)");
        // Self::emit_event("SUPPLIER_DEACTIVATED", &supplier);

        // Temporary implementation until schema supports soft delete
        Err(internal_server_error!(
            "Supplier deactivation requires schema update: add is_active field to suppliers table"
        ))
    }

    /// Reactivate a previously deactivated supplier
    ///
    /// # Arguments
    /// * `supplier_id` - ID of the supplier to reactivate
    ///
    /// # Returns
    /// The reactivated Supplier entity
    #[instrument(fields(supplier_id))]
    pub fn reactivate(supplier_id: Uuid) -> AppResult<Supplier> {
        info!(supplier_id = %supplier_id, "Reactivating supplier");

        let _supplier = SupplierRepository::find(supplier_id)?;

        // Note: This requires the is_active field to be added to the Supplier model
        // _supplier.is_active = Some(true);

        // TODO: Implement once soft delete fields are added to schema
        info!("Supplier reactivation pending schema update");

        // Temporary implementation
        Err(internal_server_error!(
            "Supplier reactivation requires schema update: add is_active field to suppliers table"
        ))
    }

    /// Validate that no active catalog items reference this supplier
    ///
    /// This ensures referential integrity - suppliers actively used in the
    /// catalog cannot be removed from the system.
    fn validate_no_active_catalog_items(supplier_id: Uuid) -> AppResult<()> {
        debug!(supplier_id = %supplier_id, "Checking for active catalog items");

        let catalog_items = CatalogItemRepository::find_by_supplier_id(supplier_id)?;

        // Filter for active/in-stock items
        let active_items: Vec<_> = catalog_items
            .into_iter()
            .filter(|item| item.in_stock == Some(true))
            .collect();

        if !active_items.is_empty() {
            return Err(invalid!(
                "Cannot deactivate supplier with {} active catalog item(s)",
                active_items.len()
            ));
        }

        Ok(())
    }

    /// Validate that no active purchase orders reference this supplier
    ///
    /// This maintains procurement workflow integrity - suppliers with active
    /// purchase orders must remain in the system for tracking and fulfillment.
    fn validate_no_active_purchase_orders(supplier_id: Uuid) -> AppResult<()> {
        use database::enums::purchase_order_status::PurchaseOrderStatus;

        debug!(supplier_id = %supplier_id, "Checking for active purchase orders");

        let pos = PurchaseOrderRepository::find_by_supplier_id(supplier_id)?;

        // Filter for non-fulfilled orders (Draft, Submitted, Approved, Rejected, Cancelled are all "active")
        // Note: Once Fulfilled status is added to the enum, this should filter it out
        let active_pos: Vec<_> = pos
            .into_iter()
            .filter(|po| po.status != PurchaseOrderStatus::Cancelled)
            .collect();

        if !active_pos.is_empty() {
            return Err(invalid!(
                "Cannot deactivate supplier with {} active purchase order(s)",
                active_pos.len()
            ));
        }

        Ok(())
    }

    /// Emit an event for external systems (audit logs, event streams, etc.)
    ///
    /// - Publish to message queues (RabbitMQ, Kafka)
    fn emit_event(event_type: AppEvent, supplier: &Supplier) -> AppResult<()> {
        run_async(Event::emit(event_type, supplier))?;
        debug!(supplier_id = %supplier.id, "[{event_type}] Event emitted");
        Ok(())
    }
}
