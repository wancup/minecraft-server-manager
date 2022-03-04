use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayInfo {
    pub url: String,
    pub api_key: String,
}

pub fn get_api_config() -> GatewayInfo {
    let conf = include_str!("../config/server.json");

    // TODO: fix unwrap
    let gateway_info: GatewayInfo = from_str(conf).unwrap();
    gateway_info
}
