use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

#[derive(Debug)]
pub enum AuthError {
    Malformed,
    Missing,
    Invalid,
}

fn is_valid(key: String) -> bool {
    key == crate::appenv::user_access_token()
}

#[derive(Debug)]
pub struct Authenticator {
    api_key: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Authenticator {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let header_values = request.headers().get("x-api-key").collect::<Vec<_>>();

        match header_values.len() {
            0 => Outcome::Failure((Status::Unauthorized, AuthError::Missing)),
            1 => {
                let parts = header_values[0]
                    .split_ascii_whitespace()
                    .collect::<Vec<_>>();

                match parts.len() {
                    0 | 1 => Outcome::Failure((Status::BadRequest, AuthError::Malformed)),
                    2 if parts[0] != "Bearer" => {
                        Outcome::Failure((Status::BadRequest, AuthError::Malformed))
                    }
                    2 if parts[0] == "Bearer" && is_valid(parts[1].to_string()) => {
                        Outcome::Success(Authenticator {
                            api_key: parts[1].to_string(),
                        })
                    }
                    2 => Outcome::Failure((Status::BadRequest, AuthError::Malformed)),
                    _ => Outcome::Failure((Status::BadRequest, AuthError::Malformed)),
                }
            }
            _ => Outcome::Failure((Status::BadRequest, AuthError::Invalid)),
        }
    }
}
