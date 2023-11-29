use std::time::Duration;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{middleware, Extension, Json, Router, Server};
use serde_json::json;
use tokio::signal;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::graphql::*;

mod api;
mod error;
mod extractor;
mod graphql;
mod guard;
mod loader;
mod prisma;
mod repository;
mod state;
mod version;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => println!("Ctrl+C received"),
        _ = terminate => println!("SIGTERM received"),
    }

    println!("signal received, starting graceful shutdown");
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "message": "404 page found"
        })),
    )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level.
                // `axum::rejection=trace` enables showing those events
                tracing_subscriber::EnvFilter::new(
                    "tokenspan_api=info,tower_http::trace::on_response=info,axum::rejection=trace",
                )
            }),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let cors_layer = CorsLayer::permissive();
    let timeout_layer = TimeoutLayer::new(Duration::from_secs(10));

    let app_state = state::AppState::init().await;
    let schema = build_schema(app_state.clone()).await;

    let app = Router::new()
        .route("/graphql", get(graphql_sandbox).post(graphql_handler))
        .nest("/api/:version", api::ApiRouter::new())
        .fallback(handler_404)
        .layer(cors_layer)
        .layer(timeout_layer)
        .layer(trace_layer)
        .layer(Extension(schema))
        .layer(middleware::from_fn(guard::guard))
        .with_state(app_state);

    println!("Sandbox: http://localhost:8080/graphql");

    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
