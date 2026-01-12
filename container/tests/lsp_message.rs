use lsp_bridge::{format_lsp_message, parse_content_length};

#[test]
fn formats_simple_message() {
    let result = format_lsp_message("hello");
    assert_eq!(result, "Content-Length: 5\r\n\r\nhello");
}

#[test]
fn formats_empty_message() {
    let result = format_lsp_message("");
    assert_eq!(result, "Content-Length: 0\r\n\r\n");
}

#[test]
fn formats_json_message() {
    let json = r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#;
    let result = format_lsp_message(json);
    assert_eq!(
        result,
        format!("Content-Length: {}\r\n\r\n{}", json.len(), json)
    );
}

#[test]
fn content_length_matches_byte_count() {
    let content = "test message with unicode: 你好";
    let result = format_lsp_message(content);
    let expected_len = content.len();
    assert!(result.starts_with(&format!("Content-Length: {}\r\n\r\n", expected_len)));
}

#[test]
fn uses_crlf_line_ending() {
    let result = format_lsp_message("test");
    assert!(result.contains("\r\n\r\n"));
    assert!(!result.contains("\n\n"));
}

#[test]
fn lsp_message_roundtrip() {
    // Simulate what happens when a message goes through formatting
    // and then is parsed back
    let original = r#"{"jsonrpc":"2.0","id":1,"method":"textDocument/hover"}"#;
    let framed = format_lsp_message(original);

    // Extract content length from the framed message
    let header_line = framed.lines().next().unwrap();
    let content_length = parse_content_length(header_line).unwrap();

    assert_eq!(content_length, original.len());

    // Extract body (after \r\n\r\n)
    let body_start = framed.find("\r\n\r\n").unwrap() + 4;
    let body = &framed[body_start..];

    assert_eq!(body, original);
}

#[test]
fn handles_multiline_json() {
    let json = r#"{
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {}
    }"#;
    let framed = format_lsp_message(json);

    let header_line = framed.lines().next().unwrap();
    let content_length = parse_content_length(header_line).unwrap();

    assert_eq!(content_length, json.len());
}
