use axum::{
    body::Body,
    extract::{FromRequestParts, WebSocketUpgrade},
    http::{HeaderMap, Request},
    response::{IntoResponse, Response},
    Router,
};

mod socket;

/// Check if request headers indicate a WebSocket upgrade
pub fn is_websocket_upgrade(headers: &HeaderMap) -> bool {
    headers
        .get("upgrade")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.eq_ignore_ascii_case("websocket"))
        .unwrap_or(false)
}

/// Parse Content-Length from an LSP header line
pub fn parse_content_length(line: &str) -> Option<usize> {
    line.strip_prefix("Content-Length:")
        .map(|len| len.trim().parse().unwrap_or(0))
}

/// Format a message with LSP Content-Length header
pub fn format_lsp_message(content: &str) -> String {
    format!("Content-Length: {}\r\n\r\n{}", content.len(), content)
}

/// Create the application router
pub fn create_app() -> Router {
    Router::new().fallback(handler)
}

async fn handler(req: Request<Body>) -> Response {
    if is_websocket_upgrade(req.headers()) {
        let (mut parts, _body) = req.into_parts();
        match WebSocketUpgrade::from_request_parts(&mut parts, &()).await {
            Ok(ws) => ws.on_upgrade(socket::handle_socket),
            Err(e) => e.into_response(),
        }
    } else {
        "OK".into_response()
    }
}
