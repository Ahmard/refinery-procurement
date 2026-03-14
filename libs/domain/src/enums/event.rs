use crate::enums::EnumError;
use std::str::FromStr;
use foxtive_macros::{impl_enum_common_traits, impl_enum_display_trait};
use crate::APP;
use crate::contracts::event_contract::EventContract;
use crate::ext::LocalAppStateExt;

impl_enum_display_trait!(AppEvent);
impl_enum_common_traits!(AppEvent);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppEvent {
    UserCreated,
    UserUpdated,
    UserDeleted,
    UserActivated,
    UserDeactivated,
    UserRoleAssigned,
    SupplierCreated,
    SupplierUpdated,
    SupplierDeleted,
    SupplierActivated,
    SupplierDeactivated,
    PurchaseOrderCreated,
    PurchaseOrderSubmitted,
    PurchaseOrderCancelled,
}

impl FromStr for AppEvent {
    type Err = EnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let event = match s {
            "proc.user.created" => AppEvent::UserCreated,
            "proc.user.updated" => AppEvent::UserUpdated,
            "proc.user.deleted" => AppEvent::UserDeleted,
            "proc.user.activated" => AppEvent::UserActivated,
            "proc.user.deactivated" => AppEvent::UserDeactivated,
            "proc.user.role.assigned" => AppEvent::UserRoleAssigned,
            "proc.supplier.created" => AppEvent::SupplierCreated,
            "proc.supplier.updated" => AppEvent::SupplierUpdated,
            "proc.supplier.deleted" => AppEvent::SupplierDeleted,
            "proc.supplier.activated" => AppEvent::SupplierActivated,
            "proc.supplier.deactivated" => AppEvent::SupplierDeactivated,
            "proc.purchase_order.created" => AppEvent::PurchaseOrderCreated,
            "proc.purchase_order.submitted" => AppEvent::PurchaseOrderSubmitted,
            "proc.purchase_order.cancelled" => AppEvent::PurchaseOrderCancelled,
            _ => return Err(EnumError::InvalidVariant(s.to_string())),
        };

        Ok(event)
    }
}

impl AppEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEvent::UserCreated => "proc.user.created",
            AppEvent::UserUpdated => "proc.user.updated",
            AppEvent::UserDeleted => "proc.user.deleted",
            AppEvent::UserActivated => "proc.user.activated",
            AppEvent::UserDeactivated => "proc.user.deactivated",
            AppEvent::UserRoleAssigned => "proc.user.role.assigned",
            AppEvent::SupplierCreated => "proc.supplier.created",
            AppEvent::SupplierUpdated => "proc.supplier.updated",
            AppEvent::SupplierDeleted => "proc.supplier.deleted",
            AppEvent::SupplierActivated => "proc.supplier.activated",
            AppEvent::SupplierDeactivated => "proc.supplier.deactivated",
            AppEvent::PurchaseOrderCreated => "proc.purchase_order.created",
            AppEvent::PurchaseOrderSubmitted => "proc.purchase_order.submitted",
            AppEvent::PurchaseOrderCancelled => "proc.purchase_order.cancelled",
        }
    }
}

impl EventContract for AppEvent {
    fn event_name(&self) -> String {
        self.as_str().to_string()
    }

    fn rmq_exchange(&self) -> &String {
        &APP.state().rmq_exchange_app
    }
}