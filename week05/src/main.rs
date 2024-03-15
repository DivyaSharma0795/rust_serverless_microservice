use lambda_runtime::{handler_fn, Context, Error};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput, AttributeValue};
use serde_json::{Value, json};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    if let Err(e) = tokio::spawn(run()).await {
        eprintln!("Error: {}", e);
    }
}

async fn run() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
    // Create a DynamoDB client
    let client = DynamoDbClient::new(Default::default());

    // Define the input for the GetItem operation
    let mut get_item_input: GetItemInput = Default::default();
    get_item_input.table_name = "YourTableName".to_string();
    get_item_input.key = HashMap::new();

    let attr_value = AttributeValue {
        s: Some(event.to_string()),
        ..Default::default()
    };
    get_item_input.key.insert("YourPrimaryKey".to_string(), attr_value);

    // Call the GetItem operation
    match client.get_item(get_item_input).await {
        Ok(output) => match output.item {
            Some(item) => Ok(json!(item)),
            None => Err("No item found".into()),
        },
        Err(error) => Err(format!("DynamoDB error: {}", error).into()),
    }
}