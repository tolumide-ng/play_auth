use argon2::{
    password_hash::{
        rand_core::OsRng, 
        PasswordHasher, SaltString, PasswordHashString
    },
    Argon2
};
use argon2::{Algorithm::Argon2id, Version::V0x13, Params};
use lazy_static::lazy_static;
use fancy_regex::Regex;

use crate::settings::variables::EnvVars;


pub struct Password(PasswordHashString);

impl Password {
    pub fn new(pwd: String, env: EnvVars) -> Option<Self> {

        let EnvVars { m_cost, p_cost, t_cost, .. } = env;

        if Password::is_valid(&pwd) {
            let salt = SaltString::generate(&mut OsRng);
            // let argon2 = Argon2::default();
            // tood()! should be secrets
            let params = Params::new(m_cost, t_cost, p_cost, None).unwrap();
            let argon2 = Argon2::new(Argon2id, V0x13, params);
            let pwd_bytes = pwd.as_bytes().clone();
            let pwd_hash = argon2.hash_password(pwd_bytes, &salt).unwrap().to_string();

            return Some(Self(PasswordHashString::new(&pwd_hash).unwrap()))
        }

        None

    }

    fn is_valid(pwd: &str) -> bool {
        // password must be atleast 8 characters with letters, numbers, and special char
        lazy_static! {
             static ref RE: Regex = Regex::new(r"^(?=.*[^a-zA-Z])(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9]\S.{8,})").unwrap();
        }

        RE.is_match(pwd).unwrap()
    }
}

