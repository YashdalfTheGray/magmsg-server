use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput};

use crate::message::Message;

pub async fn get_all_messages(
    client: DynamoDbClient,
    table_name: String,
    fields_to_get_csv: Option<String>,
) -> Vec<Message> {
    let scan_input = ScanInput {
        table_name: table_name,
        projection_expression: fields_to_get_csv,
        ..Default::default()
    };

    client
        .scan(scan_input)
        .await
        .map_or_else(
            |_| vec![],
            |response| response.items.unwrap_or_else(|| vec![]),
        )
        .iter()
        .map(|ddb_item| Message::from(ddb_item))
        .collect::<Vec<_>>()
}
