#[derive(Debug, Clone)]
pub struct Message {
    message_id: String,
    created_at: u64,
    content: String,
    created_by: String,
}
