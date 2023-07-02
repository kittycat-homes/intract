use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    Router,
};
use serde_json::{json, Value};
use serial_test::serial;
use tower::ServiceExt;

use crate::api::v1::account::RegisterData;

static ADMIN_USERNAME: &'static str = "bigboss123";
static ADMIN_PASSWORD: &'static str = "IAMBIGBOSS";

async fn app() -> Router {
    crate::generate_server().await.unwrap()
}

#[tokio::test]
async fn empty_register() {
    let app = app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/account/register")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(StatusCode::OK, response.status());
}

fn admin_register_data() -> String {
    json!(RegisterData {
        username: ADMIN_USERNAME.into(),
        password: ADMIN_PASSWORD.into(),
        join_reason: None,
    })
    .to_string()
}

async fn register_owner() -> Response {
    app()
        .await
        .oneshot(
            Request::builder()
                .uri("/api/v1/account/register")
                .body(Body::from(admin_register_data()))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
#[serial]
async fn register_new_owner() {
    assert_eq!(register_owner().await.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn try_registering_existing_account() {
    assert_ne!(register_owner().await.status(), StatusCode::OK)
}

#[tokio::test]
async fn login_unapproved() {
    
}