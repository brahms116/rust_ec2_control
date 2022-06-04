use aws_sdk_ec2::Client;
use std::error::Error;
use std::fmt::Display;

pub struct Ec2Status {
    pub state: i32,
    pub public_ip: Option<String>,
}

#[derive(Debug)]
pub enum Ec2ControlError {
    Unknown(String),
    InstanceNotFound,
}

impl Display for Ec2ControlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Ec2ControlError::Unknown(unknown) => format!("Unknown Error: {}", unknown),
            Ec2ControlError::InstanceNotFound => "Instance Not Found!".to_owned(),
        };
        write!(f, "{}", msg)
    }
}

impl Error for Ec2ControlError {}

pub fn get_ec2_status(client: &Client, instance_id: &str) -> Ec2Status {
    todo!()
}

pub fn start_ec2(client: &Client, instance_id: &str) -> Ec2Status {
    todo!()
}

pub fn stop_ec2(client: &Client, instance_id: &str) -> Ec2Status {
    todo!()
}
