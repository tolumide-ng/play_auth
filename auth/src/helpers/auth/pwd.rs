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

use crate::{settings::{app::AppSettings}};


pub struct Password(String);

impl Password {
    pub fn new(pwd: String, app: &AppSettings) -> Option<Self> {

        let AppSettings { m_cost, p_cost, t_cost, .. } = *app;

        if Password::is_valid(&pwd) {
            let salt = SaltString::generate(&mut OsRng);
            // tood()! should be secrets
            let params = Params::new(m_cost as u32, t_cost as u32, p_cost as u32, None).unwrap();
            let argon2 = Argon2::new(Argon2id, V0x13, params);
            let pwd_bytes = pwd.as_bytes().clone();
            let pwd_hash = argon2.hash_password(pwd_bytes, &salt).unwrap().to_string();

            // return Some(Self(PasswordHashString::new(&pwd_hash).unwrap()))
            return Some(Self(pwd_hash))
        }

        None

    }

    fn is_valid(pwd: &str) -> bool {
        // password must be atleast 8 characters with letters, numbers, and special char
        lazy_static! {
             static ref RE: Regex = Regex::new(r#"^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[~@#$%^&*+=`|{}:;!.?\(")\[\]-]).{8,}"#).unwrap();
        }

        RE.is_match(pwd).unwrap()
    }

    pub fn get_val(self) -> String {
        self.0
    }

    pub fn is_same(hash: String, password: String) -> bool {
        let parsed_hash = PasswordHash::new(&hash).unwrap();
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::test_helpers::get_appsettings;

    #[test]
    fn alphabets_only_password_is_invalid() {
        let pwd = "password".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_none());
    }

    #[test]
    fn numbers_only_password_is_invalid() {
        let pwd = "12345".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_none());
    }

    #[test]
    fn alphabets_and_numbers_only_is_invalid() {
        let pwd = "1234password".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_none());
    }

    #[test]
    fn password_with_short_length_is_invalid() {
        let pwd = "ATyp23*".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_none());
    }

    #[test]
    fn password_with_special_characters_only_is_invalid() {
        let pwd = "********#######".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_none());
    }

    #[test]
    fn valid_password() {
        let pwd = "Authentication1234\"".to_string();
        assert!(Password::new(pwd, &get_appsettings()).is_some());
    }
}