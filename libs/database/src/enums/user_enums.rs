use foxtive_macros::generate_diesel_enum;

generate_diesel_enum!(UserStatus {
    Active,
    Inactive,
    Suspended,
});

generate_diesel_enum!(UserRole {
    Admin,
    User,
    Supplier,
    ProcurementOfficer,
    Engineer,
    Superadmin,
});
