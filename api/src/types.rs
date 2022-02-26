use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequestType {
    StartServer,
    GetServerStatus,
    StopServer,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPayload {
    pub request_type: RequestType,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
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
}

impl From<&str> for ServerState {
    fn from(state: &str) -> Self {
        match state {
            "pending" => ServerState::Pending,
            "running" => ServerState::Running,
            "stopping" => ServerState::Stopping,
            "stopped" => ServerState::Stopped,
            _ => ServerState::Unexpected,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePayload {
    pub server_state: ServerState,
    pub ip_address: Option<String>,
}
