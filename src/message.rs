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
            message_id: crate::utils::get_string_from_attribute_value(dynamo_item.get(id_field)),
            created_at: crate::utils::get_number_from_attribute_value(
                dynamo_item.get(created_at_field),
            ),
            content: crate::utils::get_string_from_attribute_value(dynamo_item.get(content_field)),
            created_by: crate::utils::get_string_from_attribute_value(
                dynamo_item.get(created_by_field),
            ),
        }
    }
}

impl Into<HashMap<String, AttributeValue>> for Message {
    fn into(self) -> HashMap<String, AttributeValue> {
        let mut result = HashMap::<String, AttributeValue>::new();

        result.insert(
            id_field.to_string(),
            crate::utils::wrap_string_in_attribute_value(self.message_id),
        );
        result.insert(
            created_at_field.to_string(),
            crate::utils::wrap_string_in_attribute_value(self.created_at.to_string()),
        );
        result.insert(
            content_field.to_string(),
            crate::utils::wrap_string_in_attribute_value(self.content),
        );
        result.insert(
            created_by_field.to_string(),
            crate::utils::wrap_string_in_attribute_value(self.created_by),
        );

        result
    }
}
