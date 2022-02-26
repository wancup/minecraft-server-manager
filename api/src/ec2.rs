use crate::types::ResponsePayload;
use crate::types::ServerState;
use anyhow::{anyhow, Result};
use aws_sdk_ec2::Client;

#[derive(Debug)]
pub struct InstanceManager {
    client: Client,
    instance_id: String,
}

impl InstanceManager {
    pub fn new(client: Client, instance_id: String) -> Self {
        Self {
            client,
            instance_id,
        }
    }

    pub async fn check_status(&self) -> Result<ResponsePayload> {
        let response = self
            .client
            .describe_instances()
            .set_instance_ids(Some(vec![self.instance_id.clone()]))
            .send()
            .await?;
        let instance_info = response
            .reservations
            .as_ref()
            .ok_or_else(|| anyhow!("Describe Error: No Reservations"))?
            .first()
            .ok_or_else(|| anyhow!("Describe Error: No Reservation Info"))?
            .instances
            .as_ref()
            .ok_or_else(|| anyhow!("Describe Error: No Instances"))?
            .first()
            .ok_or_else(|| anyhow!("Describe Error: No Instance Info"))?;

        let ip_address = instance_info.public_ip_address.as_ref().cloned();
        let server_state: ServerState = instance_info
            .state
            .as_ref()
            .ok_or_else(|| anyhow!("Describe Error: No Instance State"))?
            .name
            .as_ref()
            .ok_or_else(|| anyhow!("Describe Error: No Instance State Name"))?
            .as_str()
            .into();
        Ok(ResponsePayload {
            ip_address,
            server_state,
        })
    }

    pub async fn start(&self) -> Result<ServerState> {
        let response = self
            .client
            .start_instances()
            .set_instance_ids(Some(vec![self.instance_id.clone()]))
            .send()
            .await?;
        let start_result = response
            .starting_instances
            .as_ref()
            .ok_or_else(|| anyhow!("Start Error: No Starting Instances"))?
            .first()
            .ok_or_else(|| anyhow!("Start Error: No Instance Info"))?
            .current_state
            .as_ref()
            .ok_or_else(|| anyhow!("Start Error: No Instance State"))?
            .name
            .as_ref()
            .ok_or_else(|| anyhow!("Start Error: No Instance State Name"))?;
        Ok(ServerState::from(start_result.as_str()))
    }

    pub async fn stop(&self) -> Result<ServerState> {
        let response = self
            .client
            .stop_instances()
            .set_instance_ids(Some(vec![self.instance_id.clone()]))
            .send()
            .await?;
        let stop_result = response
            .stopping_instances
            .as_ref()
            .ok_or_else(|| anyhow!("Stop Error: No Stopping Instances"))?
            .first()
            .ok_or_else(|| anyhow!("Stop Error: No Instance Info"))?
            .current_state
            .as_ref()
            .ok_or_else(|| anyhow!("Stop Error: No Instance State"))?
            .name
            .as_ref()
            .ok_or_else(|| anyhow!("Stop Error: No Instance State Name"))?;
        Ok(ServerState::from(stop_result.as_str()))
    }
}
