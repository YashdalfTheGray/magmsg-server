use rocket::{http::Status, State};
use rocket_contrib::json::{Json, JsonValue};
use rusoto_credential::AutoRefreshingProvider;

use crate::sdk::CustomStsProvider;
use crate::utils::determine_status;
use crate::{authenticator::Authenticator, message_request::MessageRequest};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api")]
pub fn api_index() -> JsonValue {
    determine_status()
}

#[get("/api/messages?<fields>")]
pub fn get_all_messages(
    fields: Option<String>,
    _auth: Authenticator,
    creds_provider: State<AutoRefreshingProvider<CustomStsProvider>>,
) -> JsonValue {
    let region = crate::appenv::region();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = crate::sdk::get_dynamo_client((*creds_provider.get_ref()).clone(), region);
    let messages_future = crate::dal::get_all_messages(client, crate::appenv::table_name(), fields);
    let messages = runtime.block_on(messages_future);
    json!(messages)
}

#[get("/api/messages", rank = 2)]
pub fn get_all_messages_no_auth() -> Status {
    Status::Unauthorized
}

#[put("/api/messages", format = "application/json", data = "<request_json>")]
pub fn add_new_message(
    request_json: Json<MessageRequest>,
    _auth: Authenticator,
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

#[put("/api/messages", rank = 2)]
pub fn add_new_message_no_auth() -> Status {
    Status::Unauthorized
}

#[get("/api/messages/<uuid>?<fields>")]
pub fn get_one_message(
    uuid: String,
    fields: Option<String>,
    _auth: Authenticator,
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

#[get("/api/messages/<_uuid>", rank = 2)]
pub fn get_one_message_no_auth(_uuid: String) -> Status {
    Status::Unauthorized
}
