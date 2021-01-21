use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "messageId")]
    message_id: String,
    #[serde(rename = "createdAt")]
    created_at: u64,
    #[serde(rename = "content")]
    content: String,
    #[serde(rename = "createdBy")]
    created_by: String,
}
