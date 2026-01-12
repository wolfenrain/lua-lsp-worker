use axum::http::{HeaderMap, HeaderValue};
use lsp_bridge::is_websocket_upgrade;

#[test]
fn returns_true_for_websocket_header() {
    let mut headers = HeaderMap::new();
    headers.insert("upgrade", HeaderValue::from_static("websocket"));
    assert!(is_websocket_upgrade(&headers));
}

#[test]
fn returns_true_for_websocket_header_case_insensitive() {
    let mut headers = HeaderMap::new();
    headers.insert("upgrade", HeaderValue::from_static("WebSocket"));
    assert!(is_websocket_upgrade(&headers));

    let mut headers = HeaderMap::new();
    headers.insert("upgrade", HeaderValue::from_static("WEBSOCKET"));
    assert!(is_websocket_upgrade(&headers));
}

#[test]
fn returns_false_for_empty_headers() {
    let headers = HeaderMap::new();
    assert!(!is_websocket_upgrade(&headers));
}

#[test]
fn returns_false_for_other_upgrade_values() {
    let mut headers = HeaderMap::new();
    headers.insert("upgrade", HeaderValue::from_static("h2c"));
    assert!(!is_websocket_upgrade(&headers));
}

#[test]
fn returns_false_for_missing_upgrade_header() {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("text/plain"));
    assert!(!is_websocket_upgrade(&headers));
}
