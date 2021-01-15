use rocket::http::Status;
use rocket_contrib::json::JsonValue;
use rusoto_dynamodb::DynamoDbClient;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api")]
pub fn api_index() -> JsonValue {
    json!({
        "status": "okay"
    })
}

#[get("/api/messages")]
pub fn get_all_messages() -> JsonValue {
    let client = DynamoDbClient::new(crate::appenv::region());
    let messages = crate::dal::get_all_messages(
        client,
        "messages".to_string(),
        Some("createdAt,content".to_string()),
    );
    json!([])
}

#[put("/api/messages")]
pub fn add_new_message() -> Status {
    Status::Created
}

#[get("/api/messages/<uuid>")]
pub fn get_one_message(uuid: String) -> JsonValue {
    json!({
        "messageId": uuid,
        "createdAt": 1234567890,
        "content": "this is a test message",
        "createdBy": "user"
    })
}
