use rusoto_dynamodb::{DynamoDbClient, ListTablesInput};

pub fn get_all_messages(
    client: DynamoDbClient,
    table_name: String,
    fields_to_get_csv: Option<String>,
) {
    let list_tables_input = ListTablesInput::default();
}
