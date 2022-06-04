mod ec2_control;
mod ec2_control_lambda;

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
    match inputs.action {
        RequestOperation::STATUS => {}
        RequestOperation::ON => {}
        RequestOperation::OFF => {}
    }

    // let shared_config = aws_config::load_from_env().await;
    // let client = Client::new(&shared_config);
    // let id = env::var("ID").unwrap_or("".to_owned());
    // let descrip = client
    //     .describe_instances()
    //     .instance_ids(id)
    //     .send()
    //     .await
    //     .unwrap();
    // let status = descrip
    //     .reservations()
    //     .unwrap()
    //     .get(0)
    //     .unwrap()
    //     .instances()
    //     .unwrap()
    //     .get(0)
    //     .unwrap()
    //     .state()
    //     .unwrap();
    Ok(json!(format!("")))
}
