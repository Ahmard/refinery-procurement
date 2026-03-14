use axum::Router;
use axum::routing::post;
use domain::helpers::data_loader::DataLoader;
use foxtive::helpers::block;
use foxtive_axum::http::responder::Responder;
use foxtive_axum::http::response::ext::StructResponseExt;
use foxtive_axum::http::HttpResult;
use tracing::{error, info};
use domain::dto::AuthId;

pub fn controller() -> Router {
    Router::new().route("/catalog/seed", post(seed_catalog))
}

#[utoipa::path(
    post,
    path = "/api/v1/system/catalog/seed",
    tag = "system",
    responses(
        (status = 200, description = "Catalog seeded successfully")
    )
)]
async fn seed_catalog(auth_id: AuthId) -> HttpResult {
    info!("Handling catalog seed request");

    // Path to the JSON file - adjust based on deployment
    let json_path =
        "resources/Next Round – Programming Assessment/refinery_items_50_5suppliers_strict.json";
    let result = block(move || DataLoader::load_from_file(auth_id.0, json_path)).await;

    match result {
        Ok(count) => ().respond_msg(format!("Successfully seeded {} catalog items", count)),
        Err(e) => {
            error!(error = %e, "Failed to seed catalog");
            Ok(Responder::internal_server_error_message(
                "Failed to seed catalog",
            ))
        }
    }
}
