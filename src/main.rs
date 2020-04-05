use crate::config::AWS_REGION;
use crate::server_starter::start_server;
use crate::server_status::{get_server_status, ServerState};
use crate::server_stopper::stop_server;
use lambda_runtime::error::HandlerError;
use lambda_runtime::Context;
use rusoto_ec2::Ec2Client;
use serde::{Deserialize, Serialize};

mod config;
mod server_starter;
mod server_status;
mod server_stopper;

/// POSTによるリクエストの種別
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PostRequestType {
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
struct PostPayload {
    /// リクエストの種別
    request_type: PostRequestType,
}

/// レスポンス
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponsePayload {
    server_state: ServerState,
}

/// API Gatewayからのリクエストを管理する
fn manage_request(req: PostPayload, _ctx: Context) -> Result<ResponsePayload, HandlerError> {
    match req.request_type {
        PostRequestType::StartServer => {
            let client = Ec2Client::new(AWS_REGION);
            let server_state = start_server(&client).map_err(|e| HandlerError::from(e.as_str()))?;
            Ok(ResponsePayload { server_state })
        }
        PostRequestType::GetServerStatus => {
            let client = Ec2Client::new(AWS_REGION);
            let server_state =
                get_server_status(&client).map_err(|e| HandlerError::from(e.as_str()))?;
            Ok(ResponsePayload { server_state })
        }
        PostRequestType::StopServer => {
            let client = Ec2Client::new(AWS_REGION);
            let server_state = stop_server(&client).map_err(|e| HandlerError::from(e.as_str()))?;
            Ok(ResponsePayload { server_state })
        }
    }
}

fn main() {
    lambda_runtime::lambda!(manage_request);
}
