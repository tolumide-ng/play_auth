use rocket::Request;

use super::app::ApiError;

#[catch(500)]
pub fn internal_error() -> ApiError {
    ApiError::InternalServerError
}

#[catch(400)]
pub fn bad_request(req: &Request) -> ApiError {
    ApiError::BadRequest("Bad request")
}

// #[catch(401)]
// pub fn unauthenticated(_req: &Request) -> ApiError {
//     ApiError::AuthenticationError("Authorization header is either missing or invalid")
// }