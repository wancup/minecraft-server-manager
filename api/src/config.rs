use envy::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MsmConfig {
    pub ec2_instance_id: String,
}

pub fn read_ec2_config() -> Result<MsmConfig> {
    envy::prefixed("MSM_").from_env::<MsmConfig>()
}
