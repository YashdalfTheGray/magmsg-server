use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;

enum AttributeDataType {
    B,
    BS,
    L,
    M,
    N,
    NS,
    NULL,
    S,
    SS,
}

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

fn get_data_from<T>(attr: AttributeValue, datatype: AttributeDataType) -> T {
    match datatype {
        B => attr.b.unwrap_or_default(),
        BS => attr.bs.unwrap_or_default(),
        L => attr.l.unwrap_or_default(),
        M => attr.m.unwrap_or_default(),
        N => attr.n.unwrap_or_default(),
        NS => attr.ns.unwrap_or_default(),
        NULL => attr.null.unwrap_or_default(),
        S => attr.s.unwrap_or_default(),
        SS => attr.ss.unwrap_or_default(),
    }
}
