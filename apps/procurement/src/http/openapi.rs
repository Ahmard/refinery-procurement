use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::controllers::procurement_controller::list,
        crate::http::controllers::procurement_controller::create_purchase_order,
        crate::http::controllers::procurement_controller::add_item_to_order,
        crate::http::controllers::procurement_controller::submit_order,
        crate::http::controllers::procurement_controller::get_order_details,
    ),
    tags(
        (name = "procurement", description = "Purchase Order Operations")
    )
)]
pub struct ApiDoc;
