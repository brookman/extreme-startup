use anyhow::{Ok, Result};
use axum::{
    Json, Router,
    extract::Path,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    response::IntoResponse,
    routing::get,
};
use lazy_static::lazy_static;
use regex::Regex;
use tower_http::cors::CorsLayer;

const ADDRESS: &str = "0.0.0.0:8000";

lazy_static! {
    static ref ASDF: Regex = Regex::new(r"asdf").unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    let router = create_router();
    let app = router.layer(create_cors_layer()?);

    println!("🚀 Server started successfully");

    let listener = tokio::net::TcpListener::bind(ADDRESS).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn create_cors_layer() -> Result<CorsLayer> {
    Ok(CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/upper/{text}", get(upper_handler))
}

pub async fn root_handler() -> impl IntoResponse {
    "Hello, world!"
}

pub async fn upper_handler(Path(text): Path<String>) -> impl IntoResponse {
    if ASDF.captures_iter(&text).next().is_some() {
        "42".to_string()
    } else {
        text.to_uppercase()
    }
}

pub async fn health_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Camp 2025 - Extreme startup";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
