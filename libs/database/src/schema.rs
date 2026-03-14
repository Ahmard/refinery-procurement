// @generated automatically by Diesel CLI.

diesel::table! {
    audit_logs (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        action -> Varchar,
        #[max_length = 255]
        target_entity -> Varchar,
        #[max_length = 255]
        target_id -> Varchar,
        changes -> Nullable<Jsonb>,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    auth_sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Text,
        expires_at -> Timestamp,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    catalog_item_compatibility (id) {
        id -> Uuid,
        item_id -> Uuid,
        compatible_item_id -> Uuid,
        created_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    catalog_items (id) {
        id -> Uuid,
        secondary_id -> Text,
        name -> Text,
        category -> Varchar,
        supplier_id -> Uuid,
        manufacturer -> Nullable<Text>,
        model -> Nullable<Text>,
        price_usd -> Numeric,
        lead_time_days -> Nullable<Int4>,
        in_stock -> Nullable<Bool>,
        specs -> Nullable<Jsonb>,
        created_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    purchase_order_items (id) {
        id -> Uuid,
        purchase_order_id -> Uuid,
        catalog_item_id -> Uuid,
        quantity -> Int4,
        snapshot_price -> Numeric,
        snapshot_lead_time -> Nullable<Int4>,
        created_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    purchase_order_status_history (id) {
        id -> Uuid,
        purchase_order_id -> Uuid,
        status -> Varchar,
        created_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    purchase_orders (id) {
        id -> Uuid,
        po_number -> Nullable<Text>,
        supplier_id -> Uuid,
        created_by -> Uuid,
        requestor -> Nullable<Text>,
        cost_center -> Nullable<Text>,
        payment_terms -> Nullable<Text>,
        needed_by_date -> Nullable<Date>,
        status -> Varchar,
        idempotency_key -> Nullable<Text>,
        submitted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    suppliers (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_by -> Uuid,
        name -> Text,
        contact_email -> Nullable<Text>,
        contact_phone -> Nullable<Text>,
        address -> Nullable<Text>,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        password_hash -> Text,
        role -> Varchar,
        status -> Varchar,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(audit_logs -> users (user_id));
diesel::joinable!(auth_sessions -> users (user_id));
diesel::joinable!(catalog_item_compatibility -> users (created_by));
diesel::joinable!(catalog_items -> suppliers (supplier_id));
diesel::joinable!(catalog_items -> users (created_by));
diesel::joinable!(purchase_order_items -> catalog_items (catalog_item_id));
diesel::joinable!(purchase_order_items -> purchase_orders (purchase_order_id));
diesel::joinable!(purchase_order_items -> users (created_by));
diesel::joinable!(purchase_order_status_history -> purchase_orders (purchase_order_id));
diesel::joinable!(purchase_order_status_history -> users (created_by));
diesel::joinable!(purchase_orders -> suppliers (supplier_id));
diesel::joinable!(purchase_orders -> users (created_by));
diesel::joinable!(suppliers -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    audit_logs,
    auth_sessions,
    catalog_item_compatibility,
    catalog_items,
    purchase_order_items,
    purchase_order_status_history,
    purchase_orders,
    suppliers,
    users,
);
