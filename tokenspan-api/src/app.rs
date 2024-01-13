use std::time::Duration;

use anyhow::Result;
use async_graphql_axum::GraphQLSubscription;
use axum::extract::MatchedPath;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{middleware, Extension, Json, Router};
use serde_json::json;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::configs::{AppConfig, AppEnv};
use crate::graphql::{build_schema, graphiql, graphql_handler, graphql_sandbox};
use crate::state::AppState;
use crate::{api, configs, guard};

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "message": "404 page found"
        })),
    )
}

pub fn register_tracing(env: AppEnv, config: &configs::LogConfig) {
    let trace = tracing_subscriber::registry().with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            // axum logs rejections from built-in extractors with the `axum::rejection`
            // target, at `TRACE` level.
            // `axum::rejection=trace` enables showing those events
            tracing_subscriber::EnvFilter::new(config.filter.clone())
        }),
    );

    match env {
        AppEnv::Production => {
            trace.with(tracing_subscriber::fmt::layer().json()).init();
        }
        AppEnv::Development => {
            trace.with(tracing_subscriber::fmt::layer().pretty()).init();
        }
        _ => (),
    }
}

pub async fn make_app(config: AppConfig) -> Result<Router> {
    let state = AppState::new(&config).await?;
    make_app_with_state(config, state).await
}

pub async fn make_app_with_state(config: AppConfig, state: AppState) -> Result<Router> {
    register_tracing(config.env, &config.log);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!(
                "http_request",
                method = ?request.method(),
                path,
            )
        })
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let cors_layer = CorsLayer::permissive();
    let timeout_layer = TimeoutLayer::new(Duration::from_secs(10));

    let schema = build_schema(state.clone()).await;

    let app = Router::new()
        .route("/graphql", get(graphql_sandbox).post(graphql_handler))
        .route("/graphiql", get(graphiql))
        .nest("/api/:version", api::ApiRouter::new())
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .fallback(handler_404)
        .layer(middleware::from_fn_with_state(config.clone(), guard::guard))
        .layer(cors_layer)
        .layer(timeout_layer)
        .layer(trace_layer)
        .layer(Extension(schema))
        .layer(Extension(config))
        .with_state(state);

    Ok(app)
}
