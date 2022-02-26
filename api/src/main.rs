use crate::config::read_ec2_config;
use crate::ec2::InstanceManager;
use crate::types::{RequestPayload, RequestType, ResponsePayload};
use aws_sdk_ec2::{Client, Region};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;

mod config;
mod ec2;
mod types;

async fn handle_request(event: LambdaEvent<Value>) -> Result<ResponsePayload, Error> {
    let decoded_payload = serde_json::from_value::<RequestPayload>(event.payload)?;
    let region = Region::new("ap-northeast-1");
    let aws_config = aws_config::from_env().region(region).load().await;
    let ec2_client = Client::new(&aws_config);
    let manager = InstanceManager::new(ec2_client, read_ec2_config().instance_id);

    match decoded_payload.request_type {
        RequestType::StartServer => {
            let server_state = manager.start().await?;
            Ok(ResponsePayload {
                server_state,
                ip_address: None,
            })
        }
        RequestType::GetServerStatus => {
            let response = manager.check_status().await?;
            Ok(response)
        }
        RequestType::StopServer => {
            let server_state = manager.stop().await?;
            Ok(ResponsePayload {
                server_state,
                ip_address: None,
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handle_request);
    lambda_runtime::run(func).await?;
    Ok(())
}
