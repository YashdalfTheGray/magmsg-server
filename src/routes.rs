use rocket::{http::Status, State};
use rocket_contrib::json::JsonValue;
use rusoto_credential::AutoRefreshingProvider;

use crate::sdk::CustomStsProvider;

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
pub fn get_all_messages(
    creds_provider: State<AutoRefreshingProvider<CustomStsProvider>>,
) -> JsonValue {
    let region = crate::appenv::region();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = crate::sdk::get_dynamo_client((*creds_provider.get_ref()).clone(), region);
    let messages_future = crate::dal::get_all_messages(
        client,
        crate::appenv::table_name(),
        Some("createdAt,content".to_string()),
    );
    let messages = runtime.block_on(messages_future);
    json!(messages)
}

#[put("/api/messages")]
pub fn add_new_message() -> Status {
    Status::Created
}

#[get("/api/messages/<uuid>")]
pub fn get_one_message(
    uuid: String,
    creds_provider: State<AutoRefreshingProvider<CustomStsProvider>>,
) -> JsonValue {
    let region = crate::appenv::region();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = crate::sdk::get_dynamo_client((*creds_provider.get_ref()).clone(), region);
    let message_future = crate::dal::get_one_message(
        client,
        crate::appenv::table_name(),
        uuid,
        Some("createdAt,content".to_string()),
    );
    let message = runtime.block_on(message_future);
    json!(message)
}
