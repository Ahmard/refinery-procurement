//! Repository implementations for all database models
//!
//! This module provides CRUD operations, batch operations, and custom finder methods
//! for all database entities using the `impl_crud_repo!` macro system.

pub mod audit_log_repository;
pub mod auth_session_repository;
pub mod catalog_item_compatibility_repository;
pub mod catalog_item_repository;
pub mod purchase_order_item_repository;
pub mod purchase_order_repository;
pub mod purchase_order_status_history_repository;
pub mod supplier_repository;
pub mod user_repository;

// Re-export all repositories for convenience
pub use audit_log_repository::AuditLogRepository;
pub use auth_session_repository::AuthSessionRepository;
pub use catalog_item_compatibility_repository::CatalogItemCompatibilityRepository;
pub use catalog_item_repository::CatalogItemRepository;
pub use purchase_order_item_repository::PurchaseOrderItemRepository;
pub use purchase_order_repository::PurchaseOrderRepository;
pub use purchase_order_status_history_repository::PurchaseOrderStatusHistoryRepository;
pub use supplier_repository::SupplierRepository;
pub use user_repository::UserRepository;
