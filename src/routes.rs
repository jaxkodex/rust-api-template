use axum::{
    Json, Router,
    extract::State,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    routing::get,
};
use serde_json::{Value, json};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};

use crate::{error::AppError, state::AppState};

pub fn router(state: AppState, allowed_origins: Vec<HeaderValue>) -> Router {
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    if allowed_origins.is_empty() {
        app
    } else {
        app.layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::list(allowed_origins))
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
        )
    }
}

async fn healthz() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn readyz(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map_err(|error| {
            tracing::warn!(%error, "database readiness check failed");
            AppError::ServiceUnavailable
        })?;
    Ok(Json(json!({ "status": "ready" })))
}
