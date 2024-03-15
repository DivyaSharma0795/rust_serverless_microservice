use lambda_runtime::{Context, Error};
use lambda_runtime::handler_fn;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, AttributeValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
    original_name: String,
}

#[derive(Serialize, Clone)]
pub struct CustomOutput {
    message: String,
}

async fn store_name_in_dynamodb(name: &str) -> Result<(), Error> {
    let client = DynamoDbClient::new(Region::UsEast1);
    let mut item = HashMap::new();
    item.insert(
        "name".to_string(),
        AttributeValue {
            s: Some(name.to_string()),
            ..Default::default()
        },
    );

    let put_item_input = PutItemInput {
        table_name: "names".to_string(),
        item,
        ..Default::default()
    };

    match client.put_item(put_item_input).await {
        Ok(_) => Ok(()),
        Err(error) => Err(Error::from(format!("DynamoDB error: {}", error))),
    }
}

async fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput, Error> {
    if e.first_name.is_empty() {
        return Err(Error::from("Missing first name"));
    }

    store_name_in_dynamodb(&e.first_name).await?;

    Ok(CustomOutput {
        message: format!("Hello, {}! the reverse of your name is {}!", e.original_name, e.first_name),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Check if the AWS Lambda runtime is available
    if std::env::var("AWS_EXECUTION_ENV").is_ok() {
        // If it is, use the AWS Lambda handler
        let handler = handler_fn(my_handler);
        lambda_runtime::run(handler).await?;
    } else {
        // If it's not, read input from the command line
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            return Err(Error::from("Missing first name"));
        }

        // Process the input: reverse the string
        let reversed_name = args[1].chars().rev().collect::<String>();

        // Create a test event
        let event = CustomEvent {
            first_name: reversed_name,
            original_name: args[1].clone(),
        };

        // Create a test context
        let ctx = lambda_runtime::Context::default();

        // Call the handler directly
        let result = my_handler(event, ctx).await;

        // Print the result
        match result {
            Ok(output) => println!("{}", output.message),
            Err(error) => eprintln!("Error: {}", error),
        }
    }

    Ok(())
}
