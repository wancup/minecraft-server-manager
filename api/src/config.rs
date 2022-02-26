use rusoto_core::Region;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

pub const AWS_REGION: Region = Region::ApNortheast1;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ec2Config {
    pub instance_id: String,
}

pub fn read_ec2_config() -> Ec2Config {
    let aws_config_json = include_str!("../config/aws.json");
    from_str(aws_config_json).expect("No Config File")
}
