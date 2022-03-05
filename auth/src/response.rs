use rocket::{Error, serde::json::Json};
use serde::{Serialize};

#[derive(Debug, Serialize)]
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
}

// impl<'r, T> Responder<'r, 'static> for AppResponder<T> where T: Serialize {
//     fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
//         Response::build().sized_body(5, Cursor::new(&self))
//             .header(ContentType::new("application", "x-person")).ok()
//     }
// }


// #[derive(Debug, Serialize)]
// struct Task {}

// #[catch(404)]
// fn not_found<T: Serialize>(req: &Request) -> AppResponder<T> { 
//     AppResponder{
//         code: 404,
//         body: Some(Task{}),
//         message: "".to_string()
//     }
//  }