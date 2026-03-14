//! Data Loader - Seed catalog data from JSON file
//!
//! This module provides functionality to load and seed the catalog
//! with data from the provided JSON dataset.
//!
//! # Usage
//! ```rust
//! // Load data from JSON file
//! DataLoader::load_from_file("path/to/refinery_items.json")?;
//!
//! // Or seed the database directly
//! DataLoader::seed_catalog()?;
//! ```

use crate::dto::{SupplierCreateForm, SupplierDto};
use crate::repositories::{
    CatalogItemCompatibilityRepository, CatalogItemRepository, SupplierRepository,
};
use crate::services::SupplierService;
use database::enums::catalog_category::CatalogCategory;
use database::models::catalog_item::CatalogItemInsertable;
use foxtive::prelude::AppResult;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use foxtive::invalid;
use tracing::{error, info, warn};
use uuid::Uuid;
use crate::services::catalog_item_service::CatalogItemService;

/// Raw structure matching the JSON dataset
#[derive(Debug, Deserialize)]
pub struct RefineryItemRaw {
    pub id: String,
    pub name: String,
    pub category: String,
    pub supplier: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "leadTimeDays")]
    pub lead_time_days: Option<i32>,
    #[serde(rename = "priceUsd")]
    pub price_usd: bigdecimal::BigDecimal,
    #[serde(rename = "inStock")]
    pub in_stock: Option<bool>,
    pub specs: Option<serde_json::Value>,
    #[serde(rename = "compatibleWith", default)]
    pub compatible_with: Option<Vec<String>>,
}

/// Data loader for seeding catalog data
pub struct DataLoader;

impl DataLoader {
    /// Load catalog data from a JSON file and seed the database
    ///
    /// # Arguments
    /// * `file_path` - Path to the JSON file containing refinery items
    ///
    /// # Returns
    /// Number of items seeded
    pub fn load_from_file(admin_id: Uuid, file_path: &str) -> AppResult<usize> {
        info!(file_path = %file_path, "Loading catalog data from JSON file");

        // Read JSON file
        let json_content = fs::read_to_string(file_path)
            .map_err(|e| foxtive::internal_server_error!("Failed to read JSON file: {}", e))?;

        // Parse JSON
        let raw_items: Vec<RefineryItemRaw> = serde_json::from_str(&json_content)
            .map_err(|e| foxtive::internal_server_error!("Failed to parse JSON: {}", e))?;

        info!(count = raw_items.len(), "JSON parsed successfully");

        // Seed the database
        Self::seed_catalog(admin_id, raw_items)
    }

    /// Seed the database with catalog items and suppliers
    ///
    /// # Arguments
    /// * `items` - Parsed refinery items from JSON
    ///
    /// # Returns
    /// Number of items seeded
    pub fn seed_catalog(admin_id: Uuid, items: Vec<RefineryItemRaw>) -> AppResult<usize> {
        info!(count = items.len(), "Seeding catalog data");

        // Step 1: Extract unique suppliers and create them
        let supplier_map = Self::create_suppliers(admin_id, &items)?;

        // Step 2: Create catalog items
        let item_uuid_map = Self::create_catalog_items(admin_id, &items, &supplier_map)?;

        // Step 3: Create compatibility relationships
        Self::create_compatibility_relationships(admin_id, &items, &item_uuid_map)?;

        info!("Catalog seeding completed successfully");
        Ok(items.len())
    }

    /// Create supplier records from unique suppliers in the dataset
    fn create_suppliers(
        admin_id: Uuid,
        items: &[RefineryItemRaw],
    ) -> AppResult<HashMap<String, Uuid>> {
        info!("Creating supplier records");

        let mut supplier_map = HashMap::new();

        // Extract unique supplier names
        let unique_suppliers: std::collections::HashSet<&String> =
            items.iter().map(|item| &item.supplier).collect();

        for supplier_name in unique_suppliers {
            // Skip if supplier already exists
            if SupplierRepository::exist_by_name(supplier_name)? {
                warn!(name = %supplier_name, "Supplier already exists");
                continue;
            }

            let result = SupplierService::create(SupplierDto {
                form: SupplierCreateForm {
                    name: supplier_name.clone(),
                    contact_email: format!("{supplier_name}@ahmard.com"),
                    contact_phone: None,
                    address: None,
                },
                created_by: admin_id,
            });

            // Try to create supplier, ignore if already exists
            match result {
                Ok(supplier) => {
                    info!(name = %supplier_name, uuid = %supplier.id, "Created supplier");
                    supplier_map.insert(supplier_name.clone(), supplier.id);
                }
                Err(e) => {
                    error!(name = %supplier_name, "Failed to seed supplier: {e}");
                }
            }
        }

        info!(count = supplier_map.len(), "Suppliers created");
        Ok(supplier_map)
    }

    /// Create catalog items from the dataset
    fn create_catalog_items(
        admin_id: Uuid,
        items: &[RefineryItemRaw],
        supplier_map: &HashMap<String, Uuid>,
    ) -> AppResult<HashMap<String, Uuid>> {
        info!(count = items.len(), "Creating catalog items");

        let mut item_uuid_map = HashMap::new();

        for item in items {
            // Parse category enum
            let category =
                CatalogCategory::try_from(item.category.as_str()).unwrap_or(CatalogCategory::Other);

            // Get supplier UUID
            let supplier_id = supplier_map
                .get(&item.supplier)
                .copied()
                .ok_or(invalid!("Supplier '{}' not found", item.supplier))?;

            // Create insertable item
            let insertable = CatalogItemInsertable {
                secondary_id: item.id.clone(),
                name: item.name.clone(),
                category,
                supplier_id,
                manufacturer: item.manufacturer.clone(),
                model: item.model.clone(),
                price_usd: item.price_usd.clone(),
                lead_time_days: item.lead_time_days,
                in_stock: item.in_stock,
                specs: item.specs.clone(),
                created_by: admin_id,
            };

            // Try to create item, handle duplicates
            match CatalogItemService::create(insertable) {
                Ok(created_item) => {
                    info!(
                        secondary_id = %item.id,
                        uuid = %created_item.id,
                        name = %item.name,
                        "Created catalog item"
                    );
                    item_uuid_map.insert(item.id.clone(), created_item.id);
                }
                Err(err) => {
                    // Item might already exist
                    error!(
                        secondary_id = %item.id,
                        "Failed to create catalog item: {err}"
                    );

                    // Try to find existing item by secondary_id
                    if let Ok(Some(existing)) =
                        CatalogItemRepository::find_by_secondary_id(&item.id)
                    {
                        item_uuid_map.insert(item.id.clone(), existing.id);
                    }
                }
            }
        }

        info!(count = item_uuid_map.len(), "Catalog items created");
        Ok(item_uuid_map)
    }

    /// Create compatibility relationships between items
    fn create_compatibility_relationships(
        admin_id: Uuid,
        items: &[RefineryItemRaw],
        item_uuid_map: &HashMap<String, Uuid>,
    ) -> AppResult<()> {
        info!("Creating compatibility relationships");

        let mut created_count = 0;

        for item in items {
            if let Some(compatible_with) = &item.compatible_with {
                let item_id = item_uuid_map.get(&item.id);

                if let Some(&item_uuid) = item_id {
                    for compatible_secondary_id in compatible_with {
                        let compatible_uuid = item_uuid_map.get(compatible_secondary_id);

                        if let Some(&compatible_id) = compatible_uuid {
                            // Create compatibility relationship
                            use database::models::catalog_item_compatibility::CatalogItemCompatibilityInsertable;

                            let compat = CatalogItemCompatibilityInsertable {
                                item_id: item_uuid,
                                compatible_item_id: compatible_id,
                                created_by: admin_id,
                            };

                            // Ignore duplicates
                            if CatalogItemCompatibilityRepository::create(compat).is_ok() {
                                created_count += 1;
                            }
                        } else {
                            warn!(
                                item_id = %item.id,
                                compatible_id = %compatible_secondary_id,
                                "Compatible item not found in dataset"
                            );
                        }
                    }
                }
            }
        }

        info!(count = created_count, "Compatibility relationships created");
        Ok(())
    }
}
