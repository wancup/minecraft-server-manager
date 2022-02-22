//! サーバのAPIへリクエストを送る

use crate::config::get_api_config;
use iced::Color;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::thread::sleep;
use std::time::Duration;

/// POSTによるリクエストの種別
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PostRequestType {
    /// サーバを起動する
    StartServer,
    /// サーバーの状態を取得する
    GetServerStatus,
    /// サーバを停止する
    StopServer,
}

/// POSTのPayload
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPayload {
    /// リクエストの種別
    pub request_type: PostRequestType,
}

/// サーバーの状態
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServerState {
    /// 起動処理中
    Pending,
    /// 起動中
    Running,
    /// 停止処理中
    Stopping,
    /// 停止中
    Stopped,
    /// その他予期しない状態
    Unexpected,
    /// API呼び出し中
    Connecting,
}

impl ServerState {
    pub fn color(&self) -> Color {
        match self {
            ServerState::Running => Color::from_rgb8(144, 225, 129),
            ServerState::Stopped => Color::from_rgb8(218, 135, 126),
            ServerState::Unexpected => Color::from_rgb(1.0, 0.0, 0.0),
            _ => Color::BLACK,
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        ServerState::Stopped
    }
}

impl fmt::Display for ServerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// レスポンス
#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePayload {
    pub server_state: ServerState,
    pub ip_address: Option<String>,
}

/// サーバに対してPOSTを送る
///
/// # Arguments
///
/// * payload - POST内容
/// * sleep_duration - POSTの前にスリープさせる間隔(任意)
///
pub async fn post_request_to_server(
    req_type: PostRequestType,
    sleep_duration: Option<Duration>,
) -> Result<ResponsePayload, String> {
    match sleep_duration {
        Some(time) => sleep(time),
        None => (), //NOP
    }
    let payload = PostPayload {
        request_type: req_type,
    };
    let gateway = get_api_config();
    let current_state: ResponsePayload = Client::new()
        .post(&gateway.uri)
        .header("x-api-key", &gateway.api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;
    Ok(current_state)
}
