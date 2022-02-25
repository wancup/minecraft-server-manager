use crate::config::read_ec2_config;
use crate::server_status::{get_current_sever_state_from_instance_state_change_opt, ServerState};
use rusoto_ec2::{Ec2, Ec2Client, StartInstancesRequest, StartInstancesResult};

/// サーバを起動する
///
/// # Arguments
///
/// * client - EC2クライアント
///
pub async fn start_server(client: &Ec2Client) -> Result<ServerState, String> {
    let ec2_config = read_ec2_config();
    let run_request = StartInstancesRequest {
        instance_ids: ec2_config.instance_id_list,
        additional_info: None,
        dry_run: None,
    };
    let start_result = client.start_instances(run_request).await;
    match start_result {
        Ok(ref res) => get_server_state_from_start_result(res),
        Err(ref _err) => Err("Failed to wave up server".to_string()),
    }
}

/// インスタンスの起動結果から現在のサーバの状態を取得する
///
/// # Arguments
///
/// * start_result - 起動結果
///
fn get_server_state_from_start_result(
    start_result: &StartInstancesResult,
) -> Result<ServerState, String> {
    let instance_list = &start_result.starting_instances;
    match instance_list {
        Some(list) => {
            let first_instance = list.first();
            get_current_sever_state_from_instance_state_change_opt(first_instance)
        }
        None => Err("No result to start instance".to_string()),
    }
}
