use crate::config::read_ec2_config;
use rusoto_ec2::{
    DescribeInstanceStatusRequest, DescribeInstanceStatusResult, Ec2, Ec2Client,
    InstanceStateChange, InstanceStatus,
};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

/// サーバーの状態
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

/// サーバーの状態を取得する
///
/// # Arguments
///
/// * client - EC2接続用クライアント
///
pub fn get_server_status(client: &Ec2Client) -> Result<ServerState, String> {
    let ec2_config = read_ec2_config();
    let status_request = DescribeInstanceStatusRequest {
        include_all_instances: Some(true),
        instance_ids: Some(ec2_config.instance_id_list),
        ..DescribeInstanceStatusRequest::default()
    };

    // TODO: fix unwrap
    let status_result = Runtime::new()
        .unwrap()
        .block_on(client.describe_instance_status(status_request))
        .map_err(|e| e.to_string())?;
    get_first_server_status_from_describe_status_result(&status_result)
}

/// サーバー状態一覧から最初のサーバーの状態を取得する
///
/// # Arguments
///
/// * describe_result - サーバ状態一覧
///
fn get_first_server_status_from_describe_status_result(
    describe_result: &DescribeInstanceStatusResult,
) -> Result<ServerState, String> {
    let instance_status_list = &describe_result.instance_statuses;
    match instance_status_list {
        Some(ref status_list) => {
            let first_status = status_list.first();
            match_instance_status_option_to_server_state(first_status)
        }
        None => Err("No Status of the server".to_string()),
    }
}

/// インスタンスの状態をRustEnumに変換する
///
/// # Arguments
///
/// * status_opt - インスタンスの状態
///
fn match_instance_status_option_to_server_state(
    status_opt: Option<&InstanceStatus>,
) -> Result<ServerState, String> {
    match status_opt {
        Some(status) => {
            let status_str = status
                .clone()
                .instance_state
                .ok_or("No Instance State".to_string())?
                .name
                .ok_or("No Instance Name".to_string())?;
            Ok(ServerState::from(status_str.as_str()))
        }
        None => Err("No Status in Server list".to_string()),
    }
}

/// インスタンスの状態変化結果から現在のサーバーの状態を取得する
///
/// # Arguments
///
/// * instance_state_change - 状態変化結果
///
pub fn get_current_sever_state_from_instance_state_change_opt(
    instance_state_change_opt: Option<&InstanceStateChange>,
) -> Result<ServerState, String> {
    match instance_state_change_opt {
        Some(instance_state_change) => {
            let status_str = instance_state_change
                .clone()
                .current_state
                .ok_or("No Current Instance State".to_string())?
                .name
                .ok_or("No Current Instance State Name".to_string())?;
            Ok(ServerState::from(status_str.as_str()))
        }
        None => Err("No Current Instance State In Start Result".to_string()),
    }
}
