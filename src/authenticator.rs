use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

#[derive(Debug)]
enum AuthError {
    Malformed,
    Missing,
    Invalid,
}

fn is_valid(key: String) -> bool {
    key == crate::appenv::user_access_token()
}

#[derive(Debug)]
struct Authenticator {
    api_key: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Authenticator {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        todo!()
    }
}
