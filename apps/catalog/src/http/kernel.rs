use crate::http::controllers;
use crate::http::openapi::ApiDoc;
use axum::Router;
use foxtive::FOXTIVE;
use foxtive::prelude::AppStateExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn boot() -> Router {
    let openapi_path = if FOXTIVE.env().is_production() {
        "/catalog/api-docs/openapi.json"
    } else {
        "/api-docs/openapi.json"
    };

    controllers::routes().merge(SwaggerUi::new("/swagger-ui").url(openapi_path, ApiDoc::openapi()))
}