use crate::config::AWS_REGION;
use crate::server_starter::start_server;
use crate::server_status::{get_server_status, ServerState};
use crate::server_stopper::stop_server;
use lambda_http::{service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
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
    ip_address: Option<String>,
}

impl IntoResponse for ResponsePayload {
    fn into_response(self) -> Response<Body> {
        let body = Body::Text(serde_json::to_string(&self).expect("Failed Response Serialization"));
        Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}

/// API Gatewayからのリクエストを管理する
async fn manage_request(request: Request) -> Result<impl IntoResponse, Error> {
    let decoded_payload = request
        .payload::<PostPayload>()
        .map_err(|e| format!("Invalid Payload: {}", e))?
        .ok_or_else(|| "Payload Is Empty!".to_string())?;

    match decoded_payload.request_type {
        PostRequestType::StartServer => {
            let client = Ec2Client::new(AWS_REGION);
            let server_state = start_server(&client)?;
            Ok(ResponsePayload {
                server_state,
                ip_address: None,
            })
        }
        PostRequestType::GetServerStatus => {
            let client = Ec2Client::new(AWS_REGION);
            let (server_state, ip_address) = get_server_status(&client)?;
            Ok(ResponsePayload {
                server_state,
                ip_address,
            })
        }
        PostRequestType::StopServer => {
            let client = Ec2Client::new(AWS_REGION);
            let server_state = stop_server(&client)?;
            Ok(ResponsePayload {
                server_state,
                ip_address: None,
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(manage_request)).await?;
    Ok(())
}
