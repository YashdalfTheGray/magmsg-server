use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct MessageRequest {
    message: String,
    author: String,
}
