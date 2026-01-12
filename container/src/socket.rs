use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

use crate::{format_lsp_message, parse_content_length};

pub async fn handle_socket(socket: WebSocket) {
    let (mut ws_sink, mut ws_stream) = socket.split();

    let mut child = match Command::new("/opt/bin/lua-language-server")
        .arg("--stdio")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to spawn LSP: {e}");
            return;
        }
    };

    let mut lsp_stdin = child.stdin.take().unwrap();
    let lsp_stdout = child.stdout.take().unwrap();

    // LSP stdout -> WebSocket
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);
    tokio::spawn(async move {
        let mut reader = BufReader::new(lsp_stdout);
        loop {
            // Read headers until empty line
            let mut content_length = 0usize;
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).await.unwrap_or(0) == 0 {
                    return;
                }
                if line.trim().is_empty() {
                    break;
                }
                if let Some(len) = parse_content_length(&line) {
                    content_length = len;
                }
            }

            if content_length == 0 {
                continue;
            }

            let mut body = vec![0u8; content_length];
            if reader.read_exact(&mut body).await.is_err() {
                return;
            }

            if let Ok(json) = String::from_utf8(body) {
                if tx.send(json).await.is_err() {
                    return;
                }
            }
        }
    });

    // Channel -> WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // WebSocket -> LSP stdin
    while let Some(Ok(msg)) = ws_stream.next().await {
        if let Message::Text(text) = msg {
            let framed = format_lsp_message(&text);
            if lsp_stdin.write_all(framed.as_bytes()).await.is_err()
                || lsp_stdin.flush().await.is_err()
            {
                break;
            }
        }
    }

    drop(lsp_stdin);
    let _ = child.kill().await;
    send_task.abort();
}
