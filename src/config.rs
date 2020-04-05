//! configディレクトリ以下の設定を読み込む

use rusoto_core::Region;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

pub const AWS_REGION: Region = Region::ApNortheast1;

/// EC2の設定
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ec2Config {
    /// インスタンスのIDリスト
    pub instance_id_list: Vec<String>,
}

/// EC2の設定を読み込む
pub fn read_ec2_config() -> Ec2Config {
    let aws_config_json = include_str!("../config/aws.json");
    // TODO: unwrapやめる
    from_str(aws_config_json).unwrap()
}
