use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::controllers::catalog_controller::list,
        crate::http::controllers::catalog_controller::get_item_details,
        crate::http::controllers::catalog_controller::get_compatible_items,
    ),
    tags(
        (name = "catalog", description = "Catalog Operations")
    )
)]
pub struct ApiDoc;
