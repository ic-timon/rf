//! # websocket
//!
//! websocket 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! WebSocket support

use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::response::Response;
use futures_util::StreamExt;

/// WebSocket handler function type
pub type WebSocketHandler = Box<dyn Fn(WebSocket) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync>;

/// WebSocket upgrade handler
pub async fn handle_websocket(ws: WebSocketUpgrade, handler: WebSocketHandler) -> Response {
    ws.on_upgrade(move |socket| async move {
        handler(socket).await;
    })
}

/// Default WebSocket echo handler
pub async fn echo_handler(mut socket: WebSocket) {
    while let Some(msg) = socket.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
        };

        if let Message::Text(text) = msg {
            if socket.send(Message::Text(text.clone())).await.is_err() {
                break;
            }
        } else if let Message::Close(_) = msg {
            break;
        }
    }
}
