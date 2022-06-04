mod ec2_control;
mod ec2_control_lambda;

use aws_config;
use aws_sdk_ec2::Client;
use ec2_control::{get_ec2_status, start_ec2, stop_ec2};
use ec2_control_lambda::{get_request_params, RequestOperation};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handle);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handle(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let inputs = get_request_params(event)?;
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    match inputs.action {
        RequestOperation::STATUS => {
            let result = get_ec2_status(&client, &inputs.instance_id).await?;
            return Ok(json!(format!("{:?}", result)));
        }
        RequestOperation::ON => {
            let result = start_ec2(&client, &inputs.instance_id).await?;
            return Ok(json!(format!("{:?}", result)));
        }
        RequestOperation::OFF => {
            let result = stop_ec2(&client, &inputs.instance_id).await?;
            return Ok(json!(format!("{:?}", result)));
        }
    }
}
