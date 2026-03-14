use crate::http::controllers;
use crate::http::openapi::ApiDoc;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi, Url};
use domain::is_live;

pub fn boot() -> Router {
    // The spec is always served at this path  axum app
    let spec_path = "/api-docs/openapi.json";

    // But the browser needs to request it relative to the proxy-mounted prefix
    let swagger_ui = if is_live() {
        SwaggerUi::new("/swagger-ui")
            .url(spec_path, ApiDoc::openapi())
            .config(
                Config::new([
                    Url::with_primary("Admin", "/admin/api-docs/openapi.json", true),
                    Url::new("Authentication", "/auth/api-docs/openapi.json"),
                    Url::new("Catalog", "/catalog/api-docs/openapi.json"),
                    Url::new("Procurement", "/procurement/api-docs/openapi.json"),
                ])
            )
    } else {
        SwaggerUi::new("/swagger-ui")
            .url(spec_path, ApiDoc::openapi())
    };

    controllers::routes().merge(swagger_ui)
}
