use rocket::{serde::json::Json};
use serde::{Deserialize};


#[derive(Deserialize)]
struct SignupData {
    email: &'static str,
    username: &'static str,
    password: &'static str,
}


#[post("/create")]
pub fn create(data: Json<SignupData>) {
    
}

