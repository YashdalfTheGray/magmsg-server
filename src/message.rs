use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;

#[derive(Debug, Clone)]
pub struct Message {
    message_id: String,
    created_at: u64,
    content: String,
    created_by: String,
}

impl From<HashMap<String, AttributeValue>> for Message {
    fn from(dynamo_item: HashMap<String, AttributeValue>) -> Self {
        Message {
            message_id: Default::default(),
            created_at: Default::default(),
            content: Default::default(),
            created_by: Default::default(),
        }
    }
}

impl Into<HashMap<String, AttributeValue>> for Message {
    fn into(self) -> HashMap<String, AttributeValue> {
        let result = HashMap::<String, AttributeValue>::new();

        result
    }
}
