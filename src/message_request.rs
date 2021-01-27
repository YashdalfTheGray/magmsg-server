use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct MessageRequest {
    messasge: String,
    author: String,
}
