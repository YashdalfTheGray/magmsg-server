use rocket::http::Status;
use rocket_contrib::json::JsonValue;

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
    json!([])
}

#[put("/api/messages")]
pub fn add_new_message() -> Status {
    Status::Created
}
