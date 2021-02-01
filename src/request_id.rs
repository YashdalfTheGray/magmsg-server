use rocket::{
    request::{self, FromRequest},
    Request,
};

#[derive(Debug)]
pub struct RequestId(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for &'a RequestId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(
            request.local_cache(|| RequestId(uuid::Uuid::new_v4().to_string())),
        )
    }
}
