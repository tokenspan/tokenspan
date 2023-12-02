use std::time::Duration;

use crate::configs::AppEnv;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{middleware, Extension, Json, Router};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::graphql::*;

mod api;
mod configs;
mod error;
mod extractor;
mod graphql;
mod guard;
mod loader;
mod repository;
mod state;

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "message": "404 page found"
        })),
    )
}

pub fn register_tracing(config: configs::AppConfig) {
    let trace = tracing_subscriber::registry().with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            // axum logs rejections from built-in extractors with the `axum::rejection`
            // target, at `TRACE` level.
            // `axum::rejection=trace` enables showing those events
            tracing_subscriber::EnvFilter::new(config.log.filter)
        }),
    );

    if config.env == AppEnv::Production {
        trace.with(tracing_subscriber::fmt::layer().json()).init();
    } else {
        trace.with(tracing_subscriber::fmt::layer().pretty()).init();
    }
}

#[tokio::main]
async fn main() {
    let config = configs::AppConfig::new().unwrap();

    register_tracing(config.clone());

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let cors_layer = CorsLayer::permissive();
    let timeout_layer = TimeoutLayer::new(Duration::from_secs(10));

    let app_state = state::AppState::new(config.database).await;
    let schema = build_schema(app_state.clone()).await;

    let app = Router::new()
        // .route("/graphql", get(graphql_sandbox).post(graphql_handler))
        .nest("/api/:version", api::ApiRouter::new())
        .fallback(handler_404)
        .layer(cors_layer)
        .layer(timeout_layer)
        .layer(trace_layer)
        .layer(Extension(schema))
        .layer(middleware::from_fn(guard::guard))
        .with_state(app_state);

    println!("Sandbox: http://localhost:8080/graphql");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
