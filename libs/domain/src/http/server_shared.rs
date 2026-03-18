use axum::http::{HeaderName, HeaderValue, Method};

pub fn allowed_headers() -> Vec<HeaderName> {
    vec![
        HeaderName::from_static("accept"),
        HeaderName::from_static("authorization"),
        HeaderName::from_static("content-type"),
        HeaderName::from_static("idempotency-key"),
    ]
}

pub fn allowed_methods() -> Vec<Method> {
    vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
        Method::OPTIONS,
    ]
}

pub fn allowed_origins() -> Vec<HeaderValue> {
    vec![
        HeaderValue::from_static("http://localhost:9150"),
        HeaderValue::from_static("https://proc.ahmard.com"),
    ]
}
