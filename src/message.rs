use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Message {
    message_id: String,
    created_at: u128,
    content: String,
    created_by: String,
}

impl Message {
    pub fn new(message: String, author: String) -> Self {
        Message {
            message_id: Uuid::new_v4().to_hyphenated().to_string(),
            created_at: crate::utils::get_time_in_millis(),
            content: message,
            created_by: author,
        }
    }
}

impl From<HashMap<String, AttributeValue>> for Message {
    fn from(dynamo_item: HashMap<String, AttributeValue>) -> Self {
        Message {
            message_id: dynamo_item.get("messageId").map_or_else(
                || String::default(),
                |val| val.s.clone().unwrap_or_default(),
            ),
            created_at: dynamo_item.get("createdAt").map_or_else(
                || u128::default(),
                |val| {
                    val.n
                        .clone()
                        .unwrap_or_default()
                        .parse::<u128>()
                        .unwrap_or_default()
                },
            ),
            content: dynamo_item.get("content").map_or_else(
                || String::default(),
                |val| val.s.clone().unwrap_or_default(),
            ),
            created_by: dynamo_item.get("createdBy").map_or_else(
                || String::default(),
                |val| val.s.clone().unwrap_or_default(),
            ),
        }
    }
}

impl Into<HashMap<String, AttributeValue>> for Message {
    fn into(self) -> HashMap<String, AttributeValue> {
        let result = HashMap::<String, AttributeValue>::new();

        result
    }
}
