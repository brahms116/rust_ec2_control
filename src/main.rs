use aws_sdk_ec2::Client;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

// get status
// switch off
// switch on

#[derive(Serialize, Deserialize, Debug)]
enum RequestOperation {
    STATUS,
    ON,
    OFF,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestOperationParams {
    action: RequestOperation,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handle);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handle(event: LambdaEvent<RequestOperationParams>) -> Result<Value, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let id = env::var("ID").unwrap_or("".to_owned());
    let descrip = client
        .describe_instances()
        .instance_ids(id)
        .send()
        .await
        .unwrap();
    let status = descrip
        .reservations()
        .unwrap()
        .get(0)
        .unwrap()
        .instances()
        .unwrap()
        .get(0)
        .unwrap()
        .state()
        .unwrap();
    Ok(json!(format!("{:?}", status)))
}
