use foxtive_macros::generate_diesel_enum;

generate_diesel_enum!(PurchaseOrderStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
    Cancelled,
});
