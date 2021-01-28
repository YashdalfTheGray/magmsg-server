use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct MessageRequest {
    message: String,
    author: String,
}

impl MessageRequest {
    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_author(&self) -> String {
        self.author.clone()
    }
}
