use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    Router,
};
use serde_json::json;
use serial_test::serial;
use tower::ServiceExt;

use crate::{
    api::v1::account::{LoginData, RegisterData},
    cli,
    db::models::Powerlevel,
};

static ADMIN_USERNAME: &'static str = "bigboss123";
static ADMIN_PASSWORD: &'static str = "IAMBIGBOSS";

async fn app() -> Router {
    crate::generate_server().await.unwrap()
}

async fn post_json_to(json: &str, to: &str) -> Response {
    let app = app().await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .header("Content-Type", "application/json")
                .uri(to.to_string())
                .body(Body::from(json.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    response
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
    let data = admin_register_data();
    let response = post_json_to(&data, "/api/v1/account/register");
    response.await
}

#[tokio::test]
#[serial]
async fn register_new_owner() {
    assert_eq!(register_owner().await.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn try_registering_existing_account() {
    assert_eq!(
        register_owner().await.status(),
        StatusCode::INTERNAL_SERVER_ERROR
    )
}

#[tokio::test]
#[serial]
async fn register_no_password() {
    let body = Body::from(
        json!(RegisterData {
            join_reason: None,
            username: "meow".into(),
            password: "".into()
        })
        .to_string(),
    );
    let response = app()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/account/register")
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn login_too_early() {
    let data = json!(LoginData {
        description: None,
        password: ADMIN_PASSWORD.into(),
        username: ADMIN_USERNAME.into()
    })
    .to_string();
    let response = post_json_to(&data, "/api/v1/account/login").await;
    assert_eq!(response.status(), StatusCode::IM_A_TEAPOT);
}

#[tokio::test]
#[serial]
async fn approve_user() {
    assert!(cli::user::change_powerlevel(ADMIN_USERNAME, &Powerlevel::Owner).is_ok());
}

#[tokio::test]
#[serial]
async fn login_wrong_password() {
    let data = json!(LoginData {
        description: None,
        password: "THISISNOTTHERIGHTPASSWORD".into(),
        username: ADMIN_USERNAME.into()
    })
    .to_string();
    let response = post_json_to(&data, "/api/v1/account/login").await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
