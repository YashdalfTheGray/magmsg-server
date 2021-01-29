use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

#[derive(Debug)]
enum AuthError {
    BadFormat,
    Missing,
    Invalid,
}

#[derive(Debug)]
struct Authenticator {
    api_key: String,
}

impl Authenticator {
    pub fn is_valid(&self) -> bool {
        self.api_key == crate::appenv::user_access_token()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Authenticator {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        todo!()
    }
}
