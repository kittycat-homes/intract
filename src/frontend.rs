use aide::axum::ApiRouter;
use axum::{
    body::{boxed, Full},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use hyper::{header, StatusCode, Uri};
use rust_embed::RustEmbed;

pub fn routes() -> ApiRouter {
    ApiRouter::new()
        .route("/*file", get(static_handler))
        .route("/", get(index))
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../web/dist/web/index.html"))
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.starts_with("/api/") {
        return StatusCode::NOT_FOUND.into_response();
    }

    match Assets::get(path) {
        Some(v) => {
            tracing::debug!("found file: {}", path);
            let body = boxed(Full::from(v.data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body)
                .unwrap()
        }
        // return index.html, angular router will handle the rest
        None => index().await.into_response(),
    }
}

#[derive(RustEmbed)]
#[folder = "web/dist/web/"]
struct Assets;

#[cfg(test)]
mod test {
    use hyper::{header, Body, Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn check_frontend_delivery() {
        let app = crate::generate_server().await.unwrap();
        let result = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(result.status(), StatusCode::OK);
        assert_eq!(
            result.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/html"
        );
    }

    #[tokio::test]
    async fn check_index() {
        let app = crate::generate_server().await.unwrap();
        let result = app
            .oneshot(
                Request::builder()
                    .uri("/index.html")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(result.status(), StatusCode::OK);
        assert_eq!(
            result.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/html"
        );
    }

    #[tokio::test]
    async fn check_for_api_404() {
        let app = crate::generate_server().await.unwrap();
        let result = app
            .oneshot(
                Request::builder()
                    .uri("/api/this_is_not_a_real_api_route_19839043ldjksfjksdfjsdg")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(result.status(), StatusCode::NOT_FOUND);
    }
}
