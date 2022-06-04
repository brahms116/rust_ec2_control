use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::fmt;

#[derive(Deserialize, Serialize)]
pub enum RequestOperation {
    STATUS,
    ON,
    OFF,
}

#[derive(Serialize, Deserialize)]
pub struct RequestInput {
    pub action: RequestOperation,
}

#[derive(Debug)]
pub enum Ec2ControlLambdaError {
    NoId,
    InvalidOp,
}

impl fmt::Display for Ec2ControlLambdaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Ec2ControlLambdaError::InvalidOp => {
                "Invalid Operation, key of \"action\" must be one of \"STATUS\",\"ON\" or \"OFF\""
                    .to_owned()
            }
            Ec2ControlLambdaError::NoId => {
                "No instance id set, set env var \"ID\" to the ec2_instance id".to_owned()
            }
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Ec2ControlLambdaError {}

pub struct RequestParams {
    pub action: RequestOperation,
    pub instance_id: String,
}

pub fn get_request_params(input: Value) -> Result<RequestParams, Ec2ControlLambdaError> {
    let id = std::env::var("ID").map_err(|_e| Ec2ControlLambdaError::NoId)?;
    let input = from_value::<RequestInput>(input).map_err(|_e| Ec2ControlLambdaError::InvalidOp)?;
    Ok(RequestParams {
        action: input.action,
        instance_id: id,
    })
}
