use foxtive_macros::generate_diesel_enum;

generate_diesel_enum!(SupplierStatus {
    Active,
    Inactive,
    Suspended,
});
