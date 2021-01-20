use std::collections::HashMap;

use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, ScanInput};

pub async fn get_all_messages(
    client: DynamoDbClient,
    table_name: String,
    fields_to_get_csv: Option<String>,
) -> Vec<HashMap<String, AttributeValue>> {
    let scan_input = ScanInput {
        table_name: table_name,
        projection_expression: fields_to_get_csv,
        ..Default::default()
    };

    client.scan(scan_input).await.unwrap().items.unwrap()
}
