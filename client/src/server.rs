use crate::config::get_api_config;
use iced::Color;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PostRequestType {
    StartServer,
    GetServerStatus,
    StopServer,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPayload {
    pub request_type: PostRequestType,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ServerState {
    Pending,
    Running,
    Stopping,
    #[default]
    Stopped,
    Unexpected,
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

impl fmt::Display for ServerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePayload {
    pub server_state: ServerState,
    pub ip_address: Option<String>,
}

pub async fn post_request_to_server(
    req_type: PostRequestType,
    sleep_duration: Option<Duration>,
) -> Result<ResponsePayload, String> {
    if let Some(time) = sleep_duration {
        sleep(time)
    }
    let payload = PostPayload {
        request_type: req_type,
    };
    let gateway = get_api_config();
    let current_state: ResponsePayload = Client::new()
        .post(&gateway.url)
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
