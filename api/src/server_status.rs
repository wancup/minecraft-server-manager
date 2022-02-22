use crate::config::read_ec2_config;
use rusoto_ec2::{
    DescribeInstancesRequest, DescribeInstancesResult, Ec2, Ec2Client, Instance,
    InstanceStateChange,
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

type IpAddress = Option<String>;

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
pub fn get_server_status(client: &Ec2Client) -> Result<(ServerState, IpAddress), String> {
    let ec2_config = read_ec2_config();
    let describe_request = DescribeInstancesRequest {
        instance_ids: Some(ec2_config.instance_id_list),
        ..DescribeInstancesRequest::default()
    };

    // TODO: fix unwrap
    let status_result = Runtime::new()
        .unwrap()
        .block_on(client.describe_instances(describe_request))
        .map_err(|e| e.to_string())?;
    get_first_server_status_from_describe_instances_result(&status_result)
}

/// サーバー状態一覧から最初のサーバーの状態を取得する
///
/// # Arguments
///
/// * describe_result - サーバ状態一覧
///
fn get_first_server_status_from_describe_instances_result(
    describe_result: &DescribeInstancesResult,
) -> Result<(ServerState, IpAddress), String> {
    let instance_reservation_list = &describe_result.reservations;
    match instance_reservation_list {
        Some(ref reservation_list) => {
            let first_reservation = reservation_list
                .first()
                .ok_or_else(|| "No reservation in describe result".to_string())?;
            let instance_list = first_reservation
                .clone()
                .instances
                .ok_or_else(|| "No instance in describe result".to_string())?;
            let first_instance = instance_list
                .first()
                .ok_or("No first option in describe result")?;
            get_server_state_and_address_from_instance_info(first_instance)
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
fn get_server_state_and_address_from_instance_info(
    instance: &Instance,
) -> Result<(ServerState, IpAddress), String> {
    let state = instance
        .state
        .clone()
        .ok_or_else(|| "No Instance State".to_string())?;
    let state_str = state.name.ok_or_else(|| "No State Str".to_string())?;
    let address = instance.public_ip_address.clone();
    Ok((ServerState::from(state_str.as_str()), address))
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
                .ok_or_else(|| "No Current Instance State".to_string())?
                .name
                .ok_or_else(|| "No Current Instance State Name".to_string())?;
            Ok(ServerState::from(status_str.as_str()))
        }
        None => Err("No Current Instance State In Start Result".to_string()),
    }
}
