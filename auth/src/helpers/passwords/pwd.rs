use argon2::{
    password_hash::{
        rand_core::OsRng, 
        PasswordHasher, SaltString
    },
    Argon2, PasswordHash, PasswordVerifier
};
use argon2::{Algorithm::Argon2id, Version::V0x13, Params};
use lazy_static::lazy_static;
use fancy_regex::Regex;

use crate::{settings::{app::AppSettings}, errors::app::ApiError};

#[cfg(test)]
#[path = "./pwd.test.rs"]
mod pwd_tests;

#[derive(derive_more::Display, Debug, Clone)]
pub struct Password(#[display(fmt = "{0}")]String);


impl Password {
    pub fn new(pwd: String, app: &AppSettings) -> Result<Self, ApiError> {

        let AppSettings { m_cost, p_cost, t_cost, .. } = *app;

        if Password::is_valid(&pwd) {
            let salt = SaltString::generate(&mut OsRng);
            // tood()! should be secrets
            let params = Params::new(m_cost as u32, t_cost as u32, p_cost as u32, None).unwrap();
            let argon2 = Argon2::new(Argon2id, V0x13, params);
            let pwd_bytes = pwd.as_bytes().clone();
            let pwd_hash = argon2.hash_password(pwd_bytes, &salt).unwrap().to_string();

            // return Some(Self(PasswordHashString::new(&pwd_hash).unwrap()))
            return Ok(Self(pwd_hash))
        }

        Err(ApiError::ValidationError("Password must be atleast 8 characters long containing atleast
            one special character, a capital letter, a small letter, and a digit"))

    }

    fn is_valid(pwd: &str) -> bool {
        // password must be atleast 8 characters with letters, numbers, and special char
        lazy_static! {
             static ref RE: Regex = Regex::new(r#"^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[~@#$%^&*+=`|{}:;!.?\(")\[\]-]).{8,}"#).unwrap();
        }

        RE.is_match(pwd).unwrap()
    }


    pub fn is_same(hash: String, password: String) -> bool {
        let parsed_hash = PasswordHash::new(&hash).unwrap();
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }
}

