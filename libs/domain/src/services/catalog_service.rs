//! Catalog Service - Business logic for catalog management
//!
//! This service handles all read operations for catalog items in the procurement system.
//! Catalog items represent refinery equipment, parts, and materials that can be purchased.
//!
//! # Responsibilities
//! - Searching catalog items with filters
//! - Getting detailed item information
//! - Retrieving compatible items
//! - Validating item existence
//!
//! # Business Rules
//! - Items are searched by category, supplier, stock status, or name
//! - Compatibility relationships are predefined
//! - Item details include full specifications

use crate::dto::catalog_dto::{CatalogItemDetailResponse, CatalogItemResponse};
use crate::repositories::{CatalogItemCompatibilityRepository, CatalogItemRepository};
use database::models::catalog_item::CatalogItem;
use foxtive::prelude::AppResult;
use foxtive::not_found;
use tracing::{debug, info};
use uuid::Uuid;

/// CatalogService handles all catalog read operations
///
/// This service orchestrates repository calls to provide search and retrieval
/// functionality for refinery equipment catalog.
pub struct CatalogService;

impl CatalogService {
    /// Get items compatible with a specific catalog item
    ///
    /// # Arguments
    /// * `item_id` - UUID of the catalog item
    ///
    /// # Returns
    /// List of compatible items
    pub fn get_compatible_items(item_id: Uuid) -> AppResult<Vec<CatalogItemResponse>> {
        info!(item_id = %item_id, "Getting compatible items");
        debug!("Fetching compatibility relationships");

        // Get compatible item IDs from compatibility table
        let compatible_ids = CatalogItemCompatibilityRepository::find_by_item_id(item_id)?;
        
        if compatible_ids.is_empty() {
            info!(item_id = %item_id, "No compatible items found");
            return Ok(vec![]);
        }

        // Fetch the actual compatible items
        let compatible_items: Vec<CatalogItem> = compatible_ids
            .into_iter()
            .filter_map(|compat| {
                CatalogItemRepository::find(compat.compatible_item_id).ok()
            })
            .collect();

        info!(
            item_id = %item_id, 
            count = compatible_items.len(), 
            "Compatible items retrieved"
        );

        let response: Vec<CatalogItemResponse> = compatible_items.into_iter().map(Into::into).collect();
        Ok(response)
    }

    /// Get item details with compatibility information
    ///
    /// # Arguments
    /// * `item_id` - UUID of the catalog item
    ///
    /// # Returns
    /// Detailed response including compatible items
    pub fn get_item_with_compatibility(item_id: Uuid) -> AppResult<CatalogItemDetailResponse> {
        info!(item_id = %item_id, "Getting item with compatibility info");

        let item = CatalogItemRepository::find(item_id)?;
        let compatible_items = Self::get_compatible_items(item_id)?;

        Ok(CatalogItemDetailResponse {
            item: item.into(),
            compatible_items,
        })
    }

    /// Validate that a catalog item exists
    ///
    /// # Arguments
    /// * `item_id` - UUID of the catalog item to validate
    ///
    /// # Returns
    /// Ok(()) if item exists, Error otherwise
    pub fn validate_item_exists(item_id: Uuid) -> AppResult<()> {
        debug!(item_id = %item_id, "Validating item exists");
        
        if !CatalogItemRepository::exists(item_id)? {
            return Err(not_found!("Catalog item not found"));
        }
        
        Ok(())
    }

    /// Find a catalog item by its secondary ID (from JSON dataset)
    ///
    /// # Arguments
    /// * `secondary_id` - The secondary ID string (e.g., "VLV-0101")
    ///
    /// # Returns
    /// The catalog item if found
    pub fn find_by_secondary_id(secondary_id: &str) -> AppResult<CatalogItemResponse> {
        info!(secondary_id = %secondary_id, "Finding item by secondary ID");
        
        let item = CatalogItemRepository::find_by_secondary_id(secondary_id)?
            .ok_or_else(|| not_found!("Catalog item not found"))?;
        
        Ok(item.into())
    }
}
