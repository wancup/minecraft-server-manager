//! configディレクトリ内の設定を読み込む

use serde::{Deserialize, Serialize};
use serde_json::from_str;

/// Gatewayの情報
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayInfo {
    /// API GatewayのURI
    pub uri: String,
    /// APIキー
    pub api_key: String,
}

/// サーバ管理APIに関する情報を取得する
pub fn get_api_config() -> GatewayInfo {
    let conf = include_str!("../config/server.json");

    // TODO: fix unwrap
    let gateway_info: GatewayInfo = from_str(conf).unwrap();
    gateway_info
}
