use std::io::Cursor;

use rocket::{Error, serde::json::Json, response::{Responder, self}, Request, Response};
use serde::{Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct AppResponder<T: Serialize> {
    code: u16,
    body: Option<T>,
    message: &'static str,
}

impl<T> AppResponder<T> where T: Serialize {
    pub fn reply(message: &'static str, body: Option<T>, code: u16) -> Json<Self> {
        Json(Self { message, body, code })
    }

    pub fn reply_success(body: Option<T>) -> Json<Self> {
        Json(Self{
            code: 200, message: "Success", body
        })
    }

    pub fn reply_error(body: Option<T>, code: u16) -> Json<AppResponder<T>> {
        Json(AppResponder {
            message: "Error", code, body,
        })
    }

}


#[derive(Debug, Responder)]
#[response(bound = "T: Serialize", status = 404)]
pub struct NotFoundError<T>(Json<T>);


#[derive(Debug, Responder)]
#[response(bound = "T: Serialize", status = 500)]
pub struct InternalServerError<T>(Json<T>);



// impl<'r, T> Responder<'r, 'static> for AppResponder<T> where T: Serialize {
//     fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
//         rocket_contrib::json::Json(self.clone()).respond_to(req)
//     }
// }

// #[catch(404)]
// pub fn not_found(req: &Request) -> AppResponder<&'static str> {
//     AppResponder{
//         code: 404,
//         message: "Error",
//         body: Some("Resource Not Found"),
//     }
//  }


//  #[catch(500)]
//  pub fn internal_server_error(req: &Request) -> AppResponder<&'static str> {
//      AppResponder{
//         code: 404,
//         message: "Error",
//         body: Some("Internal Server Error"),
//     }
//  }