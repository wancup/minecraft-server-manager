use crate::config::AWS_REGION;
use crate::server_starter::start_server;
use crate::server_status::{get_server_status, ServerState};
use crate::server_stopper::stop_server;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use rusoto_ec2::Ec2Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    ip_address: Option<String>,
}

/// API Gatewayからのリクエストを管理する
async fn manage_request(event: LambdaEvent<Value>) -> Result<ResponsePayload, Error> {
    let decoded_payload = serde_json::from_value::<PostPayload>(event.payload)?;

    match decoded_payload.request_type {
        PostRequestType::StartServer => {
            let client = Ec2Client::new(AWS_REGION);
            let server_state = start_server(&client).await?;
            Ok(ResponsePayload {
                server_state,
                ip_address: None,
            })
        }
        PostRequestType::GetServerStatus => {
            let client = Ec2Client::new(AWS_REGION);
            let (server_state, ip_address) = get_server_status(&client).await?;
            Ok(ResponsePayload {
                server_state,
                ip_address,
            })
        }
        PostRequestType::StopServer => {
            let client = Ec2Client::new(AWS_REGION);
            let server_state = stop_server(&client).await?;
            Ok(ResponsePayload {
                server_state,
                ip_address: None,
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(manage_request);
    lambda_runtime::run(func).await?;
    Ok(())
}
