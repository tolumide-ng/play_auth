use std::time::{SystemTime, UNIX_EPOCH, Duration};

use jsonwebtoken::{encode, EncodingKey, Header, decode, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};

use crate::settings::variables::EnvVars;

pub trait DeserializeOwned: for<'de> Deserialize<'de> {}
impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de> {}

trait Jwt where Self: Serialize + DeserializeOwned {

    fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let EnvVars {jwt_secret, ..} = EnvVars::new();
        encode(&Header::default(), &self, &EncodingKey::from_secret(&jwt_secret.as_ref()))
    }

    fn decode<T: DeserializeOwned>(token: &str) -> Result<TokenData<T>, jsonwebtoken::errors::Error> {
        let EnvVars { jwt_secret, .. } = EnvVars::new();
        decode::<T>(token, &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::new(jsonwebtoken::Algorithm::HS256))
    }
}

// on Login/ForgotPassword, we can send users this token again with the error message on their screen
// to activate their account (an email has just been sent to them), always keep record
// of the last time the verification email was sent, only send a new one after 20 
// minutes (expiry of the old verification token) - Use redis timed delete (expires and deletes after a particular time)
#[derive(Debug, Serialize, Deserialize)]
pub struct Signup {
    // we won't be saving the jwt on redis, only save the signup_id mapped to the user_id on redis
    signup_id: String,
    exp: usize,
    iat: usize,
    subj: String, // do we really need this?
}

impl Signup {
    pub fn new(signup_id: String) -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        // INFORM THE USER THAT SIGNUP TOKEN EXPIRES AFTER ONE HOUR
        let exp = SystemTime::now().checked_add(Duration::from_secs(7200)).unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;

        Self { signup_id, iat, exp, subj: "Signup".to_string(), }
    }
}

impl Jwt for Signup {}


// persist on redis ??
#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPassword {
    forgot_id: String, // unique id generated when a user requests for password change, this should be persisted
    exp: usize,
    iat: usize,
    subj: String,
}

impl Jwt for ForgotPassword {}

impl ForgotPassword {
    pub fn new(forgot_id: String) -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        let exp = SystemTime::now().checked_add(Duration::from_secs(1200)).unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        Self { forgot_id, exp, iat, subj: "Forgot".to_string(), }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    email: String,
    exp: usize,
    iat: usize,
    subj: String,
}

impl Jwt for Login {}

impl Login {
    pub fn new(email: String) -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        let exp = SystemTime::now().checked_add(Duration::from_secs(1200)).unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        Self { email, exp, iat, subj: "Login".to_string(), }
    }
}


#[cfg(test)]
mod test_signup_token {
    use super::*;

    #[test]
    fn generates_token_on_signup() {
        let token = Signup::new("signup_uuid".to_string()).encode();
        assert!(token.is_ok());
    }

    #[test]
    fn generated_signup_token_can_be_decoded() {
        let encoded_token = Signup::new("signup_uuid".to_string()).encode().unwrap();
        let decoded_token: Result<TokenData<Signup>, _> = Signup::decode(&encoded_token);

        assert!(decoded_token.is_ok());
        assert_eq!(decoded_token.unwrap().claims.subj, "Signup".to_string());
    }

    #[test]
    fn signup_token_expires_after_two_hours() {
        const TWO_HOURS: usize = 7200000; // Equivalent of two hours in ms

        let encoded_token = Signup::new("signup_uuid".to_string()).encode().unwrap();
        let decoded_token: TokenData<Signup> = Signup::decode(&encoded_token).unwrap();
        
        let active_period = decoded_token.claims.exp - decoded_token.claims.iat;
        assert_eq!(active_period, TWO_HOURS);
    }
}


#[cfg(test)]
mod test_forgot_token {
    use super::*;
    const FORGOT_UUID: &str = "forgot_uuid";

    #[test]
    fn generates_token_on_forgot() {
        let token = ForgotPassword::new(FORGOT_UUID.to_string()).encode();
        assert!(token.is_ok());
    }

    #[test]
    fn generated_forgot_token_can_be_decoded() {
        let encoded_token = ForgotPassword::new(FORGOT_UUID.to_string()).encode().unwrap();
        let decoded_token: Result<TokenData<ForgotPassword>, _> = ForgotPassword::decode(&encoded_token);

        assert!(decoded_token.is_ok());
        let token = decoded_token.unwrap();
        assert_eq!(token.claims.subj, "Forgot".to_string());
        assert_eq!(token.claims.forgot_id, FORGOT_UUID.to_string());
    }

    #[test]
    fn forgot_token_expires_after_two_hours() {
        const TWENTY_MINUTES: usize = 1200000; // Equivalent of twenty minutes in ms

        let encoded_token = ForgotPassword::new(FORGOT_UUID.to_string()).encode().unwrap();
        let decoded_token: TokenData<ForgotPassword> = ForgotPassword::decode(&encoded_token).unwrap();
        
        let active_period = decoded_token.claims.exp - decoded_token.claims.iat;
        assert_eq!(active_period, TWENTY_MINUTES);
        assert_eq!(decoded_token.claims.subj, "Forgot".to_string());

    }
}


#[cfg(test)]
mod test_login_token {
    use super::*;
    const LOGIN_EMAIL: &str = "user@ouremail.com";

    #[test]
    fn generates_token_on_login() {
        let token = Login::new(LOGIN_EMAIL.to_string()).encode();
        assert!(token.is_ok());
    }

    #[test]
    fn generated_login_token_can_be_decoded() {
        let encoded_token = Login::new(LOGIN_EMAIL.to_string()).encode().unwrap();
        let decoded_token: Result<TokenData<Login>, _> = Login::decode(&encoded_token);

        assert!(decoded_token.is_ok());
        let token = decoded_token.unwrap();
        assert_eq!(token.claims.subj, "Login".to_string());
        assert_eq!(token.claims.email, LOGIN_EMAIL.to_string());
    }

    #[test]
    fn login_token_expires_after_two_hours() {
        const TWENTY_MINUTES: usize = 1200000; // Equivalent of twenty minutes in ms

        let encoded_token = Login::new(LOGIN_EMAIL.to_string()).encode().unwrap();
        let decoded_token: TokenData<Login> = Login::decode(&encoded_token).unwrap();
        
        let active_period = decoded_token.claims.exp - decoded_token.claims.iat;
        assert_eq!(active_period, TWENTY_MINUTES);
        assert_eq!(decoded_token.claims.subj, "Login".to_string());
    }
}