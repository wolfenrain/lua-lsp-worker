use lsp_bridge::parse_content_length;

#[test]
fn parses_valid_content_length() {
    assert_eq!(parse_content_length("Content-Length: 123"), Some(123));
}

#[test]
fn parses_content_length_with_extra_whitespace() {
    assert_eq!(parse_content_length("Content-Length:   456  "), Some(456));
}

#[test]
fn parses_content_length_zero() {
    assert_eq!(parse_content_length("Content-Length: 0"), Some(0));
}

#[test]
fn returns_none_for_other_headers() {
    assert_eq!(parse_content_length("Content-Type: application/json"), None);
}

#[test]
fn returns_none_for_empty_string() {
    assert_eq!(parse_content_length(""), None);
}

#[test]
fn returns_zero_for_invalid_number() {
    assert_eq!(parse_content_length("Content-Length: abc"), Some(0));
}

#[test]
fn parses_large_content_length() {
    assert_eq!(
        parse_content_length("Content-Length: 1048576"),
        Some(1048576)
    );
}
