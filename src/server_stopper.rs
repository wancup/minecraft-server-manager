use crate::config::read_ec2_config;
use crate::server_status::{get_current_sever_state_from_instance_state_change_opt, ServerState};
use rusoto_ec2::{Ec2, Ec2Client, StopInstancesRequest, StopInstancesResult};
use tokio::runtime::Runtime;

/// サーバを停止する
///
/// # Arguments
///
/// * client - EC2クライアント
///
pub fn stop_server(client: &Ec2Client) -> Result<ServerState, String> {
    let ec2_config = read_ec2_config();
    let stop_req = StopInstancesRequest {
        instance_ids: ec2_config.instance_id_list,
        ..StopInstancesRequest::default()
    };
    let stop_result = Runtime::new()
        .unwrap()
        .block_on(client.stop_instances(stop_req));

    match stop_result {
        Ok(res) => get_server_state_from_stop_result(res),
        Err(e) => Err(e.to_string()),
    }
}

/// インスタンスの停止結果から現在のサーバーの状態を取得する
///
/// # Arguments
///
/// * stop_result - 停止結果
///
fn get_server_state_from_stop_result(
    stop_result: StopInstancesResult,
) -> Result<ServerState, String> {
    let instance_list = &stop_result.stopping_instances;
    match instance_list {
        Some(list) => {
            let first_instance = list.first();
            get_current_sever_state_from_instance_state_change_opt(first_instance)
        }
        None => Err("No result to stop instance".to_string()),
    }
}
