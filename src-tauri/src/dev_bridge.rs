//! HTTP 桥接模块
//!
//! 仅在开发模式下启用，允许浏览器 dev server 通过 HTTP 调用 Tauri 命令。

#[cfg(debug_assertions)]
pub mod dispatcher;

#[cfg(debug_assertions)]
use axum::{
    extract::State,
    http::HeaderValue,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

// 使用 Axum 服务器的 AppState
use crate::server::AppState;

#[derive(Debug, Deserialize)]
pub struct InvokeRequest {
    pub cmd: String,
    #[serde(default)]
    pub args: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct InvokeResponse {
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// 创建 HTTP 桥接路由
///
/// 仅在 dev 模式下被添加到 Axum 服务器
pub fn dev_bridge_routes() -> Router<AppState> {
    Router::new().route("/invoke", post(invoke_command)).layer(
        // CORS 配置 - 允许 localhost:1420 访问
        CorsLayer::new()
            .allow_origin("http://localhost:1420".parse::<HeaderValue>().unwrap())
            .allow_methods([axum::http::Method::POST])
            .allow_headers([axum::http::header::CONTENT_TYPE]),
    )
}

async fn invoke_command(State(state): State<AppState>, Json(req): Json<InvokeRequest>) -> Response {
    // 调用命令分发器
    match dispatcher::handle_command(&state, &req.cmd, req.args).await {
        Ok(result) => Json(InvokeResponse {
            result: Some(result),
            error: None,
        })
        .into_response(),
        Err(e) => Json(InvokeResponse {
            result: None,
            error: Some(e.to_string()),
        })
        .into_response(),
    }
}
