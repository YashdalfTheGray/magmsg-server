use rocket::{http::Status, State};
use rocket_contrib::json::{Json, JsonValue};
use rusoto_credential::AutoRefreshingProvider;

use crate::message_request::MessageRequest;
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

#[get("/api/messages?<fields>")]
pub fn get_all_messages(
    fields: Option<String>,
    creds_provider: State<AutoRefreshingProvider<CustomStsProvider>>,
) -> JsonValue {
    let region = crate::appenv::region();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = crate::sdk::get_dynamo_client((*creds_provider.get_ref()).clone(), region);
    let messages_future = crate::dal::get_all_messages(client, crate::appenv::table_name(), fields);
    let messages = runtime.block_on(messages_future);
    json!(messages)
}

#[put("/api/messages", format = "application/json", data = "<request_json>")]
pub fn add_new_message(
    request_json: Json<MessageRequest>,
    creds_provider: State<AutoRefreshingProvider<CustomStsProvider>>,
) -> Status {
    let request = request_json.into_inner();
    let region = crate::appenv::region();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = crate::sdk::get_dynamo_client((*creds_provider.get_ref()).clone(), region);
    let put_future = crate::dal::put_message(
        client,
        crate::appenv::table_name(),
        request.get_message(),
        request.get_author(),
    );
    let _put_result = runtime.block_on(put_future);

    Status::Created
}

#[get("/api/messages/<uuid>?<fields>")]
pub fn get_one_message(
    uuid: String,
    fields: Option<String>,
    creds_provider: State<AutoRefreshingProvider<CustomStsProvider>>,
) -> JsonValue {
    let region = crate::appenv::region();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = crate::sdk::get_dynamo_client((*creds_provider.get_ref()).clone(), region);
    let message_future =
        crate::dal::get_one_message(client, crate::appenv::table_name(), uuid, fields);
    let message = runtime.block_on(message_future);
    json!(message)
}
