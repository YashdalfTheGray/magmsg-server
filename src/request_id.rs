use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};

use uuid::Uuid;

#[derive(Debug)]
pub struct RequestId {}

impl Default for RequestId {
    fn default() -> Self {
        RequestId {}
    }
}

impl Fairing for RequestId {
    fn info(&self) -> Info {
        Info {
            name: "Request ID generator",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        request.local_cache(|| Uuid::new_v4().to_string());
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let id = request.local_cache(|| Uuid::nil().to_string());
        response.set_raw_header("X-Request-ID", id.clone());
    }
}
