use rocket::response::content::Json;
use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(500)]
pub fn internal_error() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Something went wrong with the request"
    })
}

#[catch(400)]
pub fn bad_request() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Bad request"
    })
}
