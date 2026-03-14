use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::controllers::admin::system_controller::seed_catalog,
        crate::http::controllers::admin::supplier_controller::index,
        crate::http::controllers::admin::supplier_controller::show,
        crate::http::controllers::admin::supplier_controller::store,
        crate::http::controllers::admin::audit_controller::get_all_logs,
        crate::http::controllers::admin::audit_controller::get_logs_by_user,
        crate::http::controllers::admin::audit_controller::get_logs_by_target,
    ),
    tags(
        (name = "admin", description = "Admin Operations"),
        (name = "system", description = "System Management"),
        (name = "suppliers", description = "Supplier Management"),
        (name = "audit", description = "Audit Logs")
    )
)]
pub struct ApiDoc;
