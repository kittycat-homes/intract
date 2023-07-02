use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    Router,
};
use serde_json::json;
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
                .method("POST")
                .uri("/api/v1/account/register")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(StatusCode::OK, response.status());
}

fn admin_register_data() -> String {
    let data = json!(RegisterData {
        password: ADMIN_PASSWORD.into(),
        username: ADMIN_USERNAME.into(),
        join_reason: None,
    })
    .to_string();
    println!("{}", data);
    data
}

async fn register_owner() -> Response {
    let body = Body::from(admin_register_data());
    let response = app()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/account/register")
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();
    println!("{:#?}", response);
    response
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
async fn login_unapproved() {}
