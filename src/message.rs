use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;
use uuid::Uuid;

const id_field: &str = "messageId";
const created_at_field: &str = "createdAt";
const content_field: &str = "content";
const created_by_field: &str = "createdBy";

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
            message_id: crate::utils::get_string_from_attribute_value(dynamo_item.get("messageId")),
            created_at: crate::utils::get_number_from_attribute_value(dynamo_item.get("createdAt")),
            content: crate::utils::get_string_from_attribute_value(dynamo_item.get("content")),
            created_by: crate::utils::get_string_from_attribute_value(dynamo_item.get("createdBy")),
        }
    }
}

impl Into<HashMap<String, AttributeValue>> for Message {
    fn into(self) -> HashMap<String, AttributeValue> {
        let result = HashMap::<String, AttributeValue>::new();

        result
    }
}
