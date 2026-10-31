use std::time::Duration;

use axum::{
    body::Body,
    http::{Request, StatusCode, header::ACCESS_CONTROL_ALLOW_ORIGIN},
};
use http_body_util::BodyExt;
use rust_api_template::{routes, state::AppState};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower::ServiceExt;

fn unreachable_pool() -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(100))
        .connect_lazy("postgres://postgres:postgres@127.0.0.1:1/template")
        .unwrap()
}

#[tokio::test]
async fn liveness_does_not_require_a_working_database() {
    let response = routes::router(
        AppState {
            db: unreachable_pool(),
        },
        vec![],
    )
    .oneshot(
        Request::builder()
            .uri("/healthz")
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .as_ref(),
        br#"{"status":"ok"}"#
    );
}

#[tokio::test]
async fn readiness_reports_an_unavailable_database_without_exposing_details() {
    let response = routes::router(
        AppState {
            db: unreachable_pool(),
        },
        vec![],
    )
    .oneshot(
        Request::builder()
            .uri("/readyz")
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .as_ref(),
        br#"{"error":"service unavailable"}"#
    );
}

#[tokio::test]
async fn cors_only_allows_configured_origins() {
    let app = routes::router(
        AppState {
            db: unreachable_pool(),
        },
        vec!["https://example.com".parse().unwrap()],
    );
    for (origin, expected) in [
        ("https://example.com", Some("https://example.com")),
        ("https://other.example", None),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/healthz")
                    .header("origin", origin)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            response
                .headers()
                .get(ACCESS_CONTROL_ALLOW_ORIGIN)
                .and_then(|value| value.to_str().ok()),
            expected
        );
    }
}

#[sqlx::test]
async fn readiness_succeeds_when_postgres_is_available(pool: PgPool) {
    let response = routes::router(AppState { db: pool }, vec![])
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .as_ref(),
        br#"{"status":"ready"}"#
    );
}
