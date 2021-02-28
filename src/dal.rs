use std::collections::HashMap;

use crate::constants::*;
use crate::message::Message;
use rusoto_core::RusotoError;
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemError, PutItemInput,
    PutItemOutput, ScanInput,
};
use rusoto_s3::{PutObjectError, PutObjectOutput, PutObjectRequest, S3Client, StreamingBody, S3};

pub async fn get_all_messages(
    client: DynamoDbClient,
    table_name: String,
    fields_to_get_csv: Option<String>,
) -> Vec<Message> {
    let scan_input = ScanInput {
        table_name,
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

pub async fn get_one_message(
    client: DynamoDbClient,
    table_name: String,
    message_id: String,
    fields_to_get_csv: Option<String>,
) -> Message {
    let mut key = HashMap::new();
    key.insert(
        ID_FIELD.to_string(),
        AttributeValue {
            s: Some(message_id.to_string()),
            ..Default::default()
        },
    );

    let get_item_input = GetItemInput {
        table_name,
        key,
        projection_expression: fields_to_get_csv,
        ..Default::default()
    };

    client.get_item(get_item_input).await.map_or_else(
        |_| Message::not_found(message_id.to_string()),
        |response| match response.item {
            Some(item) => Message::from(item),
            None => Message::not_found(message_id.to_string()),
        },
    )
}

pub async fn put_message(
    client: DynamoDbClient,
    table_name: String,
    message: String,
    author: String,
) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    let message_to_put = Message::new(message, author);
    let ddb_item: HashMap<String, AttributeValue> = message_to_put.clone().into();
    let put_item_input = PutItemInput {
        table_name,
        item: ddb_item,
        ..Default::default()
    };

    client.put_item(put_item_input).await
}

pub async fn put_object(
    client: S3Client,
    bucket_name: String,
    key: String,
    content: String,
) -> Result<PutObjectOutput, RusotoError<PutObjectError>> {
    let put_object_request = PutObjectRequest {
        bucket: bucket_name,
        body: Some(StreamingBody::from(content.clone().as_bytes().to_vec())),
        key,
        ..Default::default()
    };

    client.put_object(put_object_request).await
}
