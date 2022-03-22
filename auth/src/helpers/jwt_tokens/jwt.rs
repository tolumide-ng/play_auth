use std::time::{SystemTime, UNIX_EPOCH, Duration};

use auth_macro_derive::JwtHelper;
use auth_macro::jwt::JwtHelper;
use jsonwebtoken::{encode, EncodingKey, Header, decode, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::errors::app::ApiError;
use crate::settings::{app::AppSettings};
use crate::helpers::mails::email::ValidEmail;

use crate::helpers::commons::{MINUTES_120, MINUTES_20};

#[cfg(test)]
#[path = "./jwt.test.rs"]
mod jwt_test;

pub trait DeserializeOwned: for<'de> Deserialize<'de> {}
impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de> {}

pub trait Jwt where Self: Serialize + DeserializeOwned {

    fn encode(&self, app: &AppSettings) -> Result<String, ApiError> {
        let AppSettings { jwt_secret, ..} = app;
        let token = encode(&Header::default(), &self, &EncodingKey::from_secret(&jwt_secret.as_ref()))?;
        
        Ok(token)
    }

    fn decode<T: DeserializeOwned>(token: &str, app: &AppSettings) -> Result<TokenData<T>, ApiError> {
        let AppSettings { jwt_secret, .. } = app;
        let data = decode::<T>(token, &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::new(jsonwebtoken::Algorithm::HS256))?;

        Ok(data)
    }
}

#[derive(Debug, Serialize, Deserialize, JwtHelper)]
pub struct SignupJwt {
    user_id: Uuid,
    exp: usize,
    iat: usize,
    subj: String, // do we really need this?
}

impl SignupJwt {
    pub fn new(user_id: Uuid) -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        // INFORM THE USER THAT SIGNUP TOKEN EXPIRES AFTER TWO HOURS
        let exp = SystemTime::now().checked_add(Duration::from_secs(MINUTES_120)).unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;

        Self { user_id, iat, exp, subj: "Signup".to_string(), }
    }
}

impl Jwt for SignupJwt {}


// persist on redis ??
#[derive(Debug, Serialize, Deserialize, JwtHelper)]
pub struct ForgotPasswordJwt {
    user_id: Uuid, // unique id generated when a user requests for password change, this should be persisted
    exp: usize,
    iat: usize,
    subj: String,
}

impl Jwt for ForgotPasswordJwt {}

impl ForgotPasswordJwt {
    pub fn new(user_id: Uuid) -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        // 20 minutes
        let exp = SystemTime::now().checked_add(Duration::from_secs(MINUTES_20)).unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        Self { user_id, exp, iat, subj: "Forgot".to_string(), }
    }
}


#[derive(Debug, Serialize, Deserialize, JwtHelper)]
pub struct LoginJwt {
    email: String,
    user_id: Uuid,
    verified: bool,
    context: String,
    exp: usize,
    iat: usize,
    subj: String,
}

impl LoginJwt {
    pub fn new(email: ValidEmail, user_id: Uuid, context: String, verified: bool) -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        // 20 minutes
        let exp = SystemTime::now().checked_add(Duration::from_secs(MINUTES_20)).unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        Self { email: email.to_string(), exp, iat, subj: "Login".to_string(), user_id, context, verified }
    }

    pub fn email(&self) -> String {
        self.email.to_string()
    }
}

impl Jwt for LoginJwt {}

