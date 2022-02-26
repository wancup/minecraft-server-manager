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
        let instance_info = self
            .client
            .describe_instances()
            .set_instance_ids(Some(vec![self.instance_id.clone()]))
            .send()
            .await?
            .reservations
            .as_ref()
            .ok_or_else(|| anyhow!("Describe Error: No Reservations"))?
            .first()
            .ok_or_else(|| anyhow!("Describe Error: No Reservation Info"))?
            .instances
            .as_ref()
            .ok_or_else(|| anyhow!("Describe Error: No Instances"))?
            .first()
            .ok_or_else(|| anyhow!("Describe Error: No Instance Info"))?
            .clone();

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
        let start_result = self
            .client
            .start_instances()
            .set_instance_ids(Some(vec![self.instance_id.clone()]))
            .send()
            .await?
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
            .ok_or_else(|| anyhow!("Start Error: No Instance State Name"))?
            .clone();
        Ok(ServerState::from(start_result.as_str()))
    }

    pub async fn stop(&self) -> Result<ServerState> {
        let stop_result = self
            .client
            .stop_instances()
            .set_instance_ids(Some(vec![self.instance_id.clone()]))
            .send()
            .await?
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
            .ok_or_else(|| anyhow!("Stop Error: No Instance State Name"))?
            .clone();
        Ok(ServerState::from(stop_result.as_str()))
    }
}
