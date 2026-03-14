//! Data Transfer Objects for all domain operations
//!
//! This module defines the structure of data transferred between layers.

pub mod audit_log_dto;
pub mod auth_dto;
pub mod catalog_dto;
pub mod purchase_order_dto;
pub mod supplier_dto;
pub mod user_dto;

// Re-export commonly used types for convenience
pub use audit_log_dto::*;
pub use auth_dto::*;
pub use catalog_dto::*;
pub use purchase_order_dto::*;
pub use supplier_dto::*;
pub use user_dto::*;
