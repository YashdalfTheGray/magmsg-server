use rocket::request::FromRequest;

struct Authenticator {
    api_key: String,
}

impl Authenticator {
    pub fn is_valid(&self) -> bool {
        self.api_key == crate::appenv::user_access_token()
    }
}
