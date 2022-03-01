use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, 
        PasswordHasher, SaltString
    },
    Argon2
};

use argon2::{Algorithm::Argon2id, Version::V0x13, Params};

struct Password<'a>(PasswordHash<'a>);

impl<'a> Password<'a> {
    pub fn new(pwd: String) -> Option<Self> {
        // let pwd= 

        if Password::is_valid(pwd) {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            // totod()! should be secrets
            let params = Params::new(15000, 2, 1, None).unwrap();
            let argon2 = Argon2::new(Argon2id, V0x13, params);
            let pwd_hash = argon2.hash_password(pwd.as_bytes(), &salt).unwrap();
            return Some(Self(pwd_hash))
        }

        None

    }

    fn is_valid(pwd: String) -> bool {
        // password must be atleast 8 characters with letters, numbers, and special char
        // todo() - learn regex in rust
        true
    }
}