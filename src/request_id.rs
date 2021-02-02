use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    request::{self, FromRequest},
    Data, Request, Response,
};

#[derive(Debug)]
pub struct RequestId(pub String);

impl RequestId {
    pub fn nil() -> RequestId {
        RequestId(uuid::Uuid::nil().to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &'a RequestId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(
            request.local_cache(|| RequestId(uuid::Uuid::new_v4().to_string())),
        )
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
        request.local_cache(|| RequestId(uuid::Uuid::new_v4().to_string()));
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let id = request.local_cache(|| RequestId::nil());
        response.set_raw_header("X-Request-ID", id.0.clone());
    }
}
