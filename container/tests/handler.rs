use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use lsp_bridge::create_app;
use tower::ServiceExt;

#[tokio::test]
async fn returns_ok_for_non_websocket_request() {
    let app = create_app();

    let request = Request::builder().uri("/").body(Body::empty()).unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"OK");
}

#[tokio::test]
async fn returns_ok_for_any_path() {
    let app = create_app();

    let request = Request::builder()
        .uri("/some/random/path")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn returns_ok_for_post_request() {
    let app = create_app();

    let request = Request::builder()
        .method("POST")
        .uri("/")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn handles_request_with_various_headers() {
    let app = create_app();

    let request = Request::builder()
        .uri("/")
        .header("Content-Type", "application/json")
        .header("Accept", "text/html")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
