use rocket::{serde::json::Json};
use serde::{Serialize};


#[derive(Debug, Serialize, Clone)]
pub struct ApiSuccess<T: Serialize> {
    pub status: u16,
    pub body: Option<T>,
    pub message: &'static str,
}

impl<T> ApiSuccess<T> where T: Serialize {
    pub fn reply(message: &'static str, body: Option<T>, status: u16) -> Json<Self> {
        Json(Self { message, body, status })
    }

    pub fn reply_success(body: Option<T>) -> Json<Self> {
        Json(Self{
            status: 200, message: "Success", body
        })
    }

    pub fn reply_error(body: Option<T>, status: u16) -> Json<ApiSuccess<T>> {
        Json(ApiSuccess {
            message: "Error", status, body,
        })
    }

}

// #[derive(Debug, Serialize, Deserialize)]
// pub enum SuccessResponse<T> {
//     #[serde(rename = "error")]
//     Response {
//         status: u16,
//         message: &'static str,
//         #[serde(skip_serializing_if = "Option::is_none")]
//         body: Option<T>
//     }
// }



// impl<'r, T> Responder<'r, 'static> for AppResponder<T> where T: Serialize {
//     fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
//         let success_response = SuccessResponse::Response {
//             status: self.code,
//             message: self.message,
//             body: Some(self.body),
//         };

//         let d = StatusCode::OK;

//         let response = Response::build_from(Json(success_response).respond_to(request)?)
//             .status(Status {status: 200})
//             .finalize();

//         Ok(response)
//     }
// }


