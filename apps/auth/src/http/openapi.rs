use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        domain::http::controllers::auth_controller::login,
        domain::http::controllers::auth_controller::me,
        crate::http::controllers::admin::system_controller::seed_catalog,
        crate::http::controllers::admin::supplier_controller::index,
        crate::http::controllers::admin::supplier_controller::show,
        crate::http::controllers::admin::supplier_controller::store,
    ),
    tags(
        (name = "auth", description = "Authentication Operations"),
        (name = "system", description = "System Management"),
        (name = "suppliers", description = "Supplier Management")
    )
)]
pub struct ApiDoc;
