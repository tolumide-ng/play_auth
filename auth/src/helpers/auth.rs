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
    pub fn new(pwd: String) -> Option<Self> {

        let EnvVars { m_cost, p_cost, t_cost, .. } = EnvVars::new();

        if Password::is_valid(&pwd) {
            let salt = SaltString::generate(&mut OsRng);
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
             static ref RE: Regex = Regex::new(r#"^(?=.*[a-z])(?=.*[A-Z])(?=.*[~@#$%^&*+=`|{}:;!.?\(")\[\]-]).{8,}"#).unwrap();
        }

        RE.is_match(pwd).unwrap()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabets_only_password_is_invalid() {
        let pwd = "password".to_string();
        assert!(Password::new(pwd).is_none());
    }

    #[test]
    fn numbers_only_password_is_invalid() {
        let pwd = "12345".to_string();
        assert!(Password::new(pwd).is_none());
    }

    #[test]
    fn alphabets_and_numbers_only_is_invalid() {
        let pwd = "1234password".to_string();
        assert!(Password::new(pwd).is_none());
    }

    #[test]
    fn password_with_short_length_is_invalid() {
        let pwd = "ATyp23*".to_string();
        assert!(Password::new(pwd).is_none());
    }

    #[test]
    fn password_with_special_characters_only_is_invalid() {
        let pwd = "********#######".to_string();
        assert!(Password::new(pwd).is_none());
    }

    #[test]
    fn valid_password() {
        let pwd = "Authentication1234\"".to_string();
        assert!(Password::new(pwd).is_some());
    }
}