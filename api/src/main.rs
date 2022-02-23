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
fn manage_request(req: PostPayload) -> Result<ResponsePayload, Response<Body>> {
    let handle_500_error = |e: String| {
        Response::builder()
            .status(500)
            .body(Body::Text(e))
            .expect("failed to render response")
    };

    match req.request_type {
        PostRequestType::StartServer => {
            let client = Ec2Client::new(AWS_REGION);
            let server_state = start_server(&client).map_err(handle_500_error)?;
            Ok(ResponsePayload {
                server_state,
                ip_address: None,
            })
        }
        PostRequestType::GetServerStatus => {
            let client = Ec2Client::new(AWS_REGION);
            let (server_state, ip_address) =
                get_server_status(&client).map_err(handle_500_error)?;
            Ok(ResponsePayload {
                server_state,
                ip_address,
            })
        }
        PostRequestType::StopServer => {
            let client = Ec2Client::new(AWS_REGION);
            let server_state = stop_server(&client).map_err(handle_500_error)?;
            Ok(ResponsePayload {
                server_state,
                ip_address: None,
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = move |event: Request| async move {
        let decoded_payload = event.payload::<PostPayload>();

        let response = match decoded_payload {
            Ok(Some(payload)) => manage_request(payload)
                .map(|r| r.into_response())
                .unwrap_or_else(|e| e),
            Ok(None) => Response::builder()
                .status(400)
                .body("Payload Is Empty".into())
                .expect("failed to render response"),
            Err(e) => Response::builder()
                .status(400)
                .body(format!("Invalid Payload Format: {:?}", e).into())
                .expect("failed to render response"),
        };
        Ok(response)
    };

    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}
