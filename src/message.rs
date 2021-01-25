use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;
use uuid::Uuid;

const ID_FIELD: &str = "messageId";
const CREATED_AT_FIELD: &str = "createdAt";
const CONTENT_FIELD: &str = "content";
const CREATED_BY_FIELD: &str = "createdBy";

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

impl From<&HashMap<String, AttributeValue>> for Message {
    fn from(dynamo_item: &HashMap<String, AttributeValue>) -> Self {
        Message {
            message_id: crate::utils::get_string_from_attribute_value(dynamo_item.get(ID_FIELD)),
            created_at: crate::utils::get_number_from_attribute_value(
                dynamo_item.get(CREATED_AT_FIELD),
            ),
            content: crate::utils::get_string_from_attribute_value(dynamo_item.get(CONTENT_FIELD)),
            created_by: crate::utils::get_string_from_attribute_value(
                dynamo_item.get(CREATED_BY_FIELD),
            ),
        }
    }
}

impl Into<HashMap<String, AttributeValue>> for Message {
    fn into(self) -> HashMap<String, AttributeValue> {
        let mut result = HashMap::<String, AttributeValue>::new();

        result.insert(
            ID_FIELD.to_string(),
            crate::utils::wrap_string_in_attribute_value(self.message_id),
        );
        result.insert(
            CREATED_AT_FIELD.to_string(),
            crate::utils::wrap_number_in_attribute_value(self.created_at),
        );
        result.insert(
            CONTENT_FIELD.to_string(),
            crate::utils::wrap_string_in_attribute_value(self.content),
        );
        result.insert(
            CREATED_BY_FIELD.to_string(),
            crate::utils::wrap_string_in_attribute_value(self.created_by),
        );

        result
    }
}
