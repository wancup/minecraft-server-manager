use crate::types::ResponsePayload;
use crate::types::ServerState;
use anyhow::{anyhow, Result};
use rusoto_ec2::{
    DescribeInstancesRequest, Ec2, Ec2Client, StartInstancesRequest, StopInstancesRequest,
};

pub struct InstanceManager {
    client: Ec2Client,
    instance_id: String,
}

impl InstanceManager {
    pub fn new(client: Ec2Client, instance_id: String) -> Self {
        Self {
            client,
            instance_id,
        }
    }

    pub async fn check_status(&self) -> Result<ResponsePayload> {
        let describe_request = DescribeInstancesRequest {
            instance_ids: Some(vec![self.instance_id.clone()]),
            ..DescribeInstancesRequest::default()
        };

        let instance_info = self
            .client
            .describe_instances(describe_request)
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
        let run_request = StartInstancesRequest {
            instance_ids: vec![self.instance_id.clone()],
            additional_info: None,
            dry_run: None,
        };
        let start_result = self
            .client
            .start_instances(run_request)
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
        let stop_req = StopInstancesRequest {
            instance_ids: vec![self.instance_id.clone()],
            ..StopInstancesRequest::default()
        };
        let stop_result = self
            .client
            .stop_instances(stop_req)
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
