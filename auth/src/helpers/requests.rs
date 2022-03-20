use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

pub struct AuthUser<'r>(&'r str);

#[derive(Debug)]
pub enum AuthUserError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser<'r> {
    type Error = AuthUserError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Failure((Status::BadRequest, AuthUserError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(AuthUser(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, AuthUserError::Invalid)),
        }
    }
}