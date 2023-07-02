use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    Router,
};
use serde_json::json;
use serial_test::serial;
use tower::ServiceExt;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{filter::Directive, fmt::SubscriberBuilder, EnvFilter};

use crate::{
    api::v1::account::{LoginData, RegisterData},
    cli,
    db::{models::Powerlevel},
};

static ADMIN_USERNAME: &str = "bigboss123";
static ADMIN_PASSWORD: &str = "IAMBIGBOSS";

async fn app() -> Router {
    crate::generate_server().await.unwrap()
}

async fn post_json_to(json: &str, to: &str) -> Response {
    let app = app().await;
    
    app
        .oneshot(
            Request::builder()
                .method("POST")
                .header("Content-Type", "application/json")
                .uri(to.to_string())
                .body(Body::from(json.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
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

#[test]
#[serial]
fn enable_tracing() {
    let d: Directive = LevelFilter::DEBUG.into();
    // enable tracing
    SubscriberBuilder::default()
        .pretty()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(d)
                .from_env_lossy(),
        )
        .init();
}

#[tokio::test]
async fn integration_test() {
    register_new_owner().await;
    try_registering_existing_account().await;
    register_no_password().await;
    login_too_early().await;
    approve_user().await;
    login_wrong_password().await;
}

async fn register_new_owner() {
    assert_eq!(register_owner().await.status(), StatusCode::OK);
}

async fn try_registering_existing_account() {
    assert_eq!(
        register_owner().await.status(),
        StatusCode::INTERNAL_SERVER_ERROR
    )
}

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

async fn approve_user() {
    assert!(cli::user::change_powerlevel(ADMIN_USERNAME, &Powerlevel::Owner).is_ok());
}

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
