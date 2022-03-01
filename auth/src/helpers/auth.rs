use std::borrow::Cow;

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, 
        PasswordHasher, SaltString
    },
    Argon2
};

use argon2::{Algorithm::Argon2id, Version::V0x13, Params};

use crate::settings::variables::EnvVars;

struct Password(String);

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

            return Some(Self(pwd_hash))
        }

        None

    }

    fn is_valid(pwd: &String) -> bool {
        // password must be atleast 8 characters with letters, numbers, and special char
        // todo() - learn regex in rust
        true
    }
}