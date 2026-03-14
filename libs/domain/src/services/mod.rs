//! Service layer implementations for business logic
//!
//! Services contain all business logic, validations, and orchestrate repository operations.
//! Only mutation operations are exposed through services.

pub mod audit_log_service;
pub mod auth_service;
pub mod catalog_item_service;
pub mod catalog_service;
pub mod purchase_order_service;
pub mod supplier_service;
pub mod user_service;

// Re-export services for convenience
pub use audit_log_service::AuditLogService;
pub use auth_service::AuthService;
pub use catalog_service::CatalogService;
pub use purchase_order_service::PurchaseOrderService;
pub use supplier_service::SupplierService;
pub use user_service::UserService;
