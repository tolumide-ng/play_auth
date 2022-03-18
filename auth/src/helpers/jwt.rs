use std::time::{SystemTime, UNIX_EPOCH, Duration};

use jsonwebtoken::{encode, EncodingKey, Header, decode, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::settings::{app::AppSettings};
use crate::helpers::mail::ValidEmail;

pub trait DeserializeOwned: for<'de> Deserialize<'de> {}
impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de> {}

pub trait Jwt where Self: Serialize + DeserializeOwned {

    fn encode(&self, app: &AppSettings) -> Result<String, jsonwebtoken::errors::Error> {
        let AppSettings { jwt_secret, ..} = app;
        encode(&Header::default(), &self, &EncodingKey::from_secret(&jwt_secret.as_ref()))
    }

    fn decode<T: DeserializeOwned>(token: &str, app: &AppSettings) -> Result<TokenData<T>, jsonwebtoken::errors::Error> {
        let AppSettings { jwt_secret, .. } = app;
        decode::<T>(token, &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::new(jsonwebtoken::Algorithm::HS256))
    }
}

// on Login/ForgotPassword, we can send users this token again with the error message on their screen
// to activate their account (an email has just been sent to them), always keep record
// of the last time the verification email was sent, only send a new one after 20 
// minutes (expiry of the old verification token) - Use redis timed delete (expires and deletes after a particular time)
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupJwt {
    // we won't be saving the jwt on redis, only save the signup_id mapped to the user_id on redis
    signup_id: Uuid,
    exp: usize,
    iat: usize,
    subj: String, // do we really need this?
}

impl SignupJwt {
    pub fn new(signup_id: Uuid) -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        // INFORM THE USER THAT SIGNUP TOKEN EXPIRES AFTER ONE HOUR
        let exp = SystemTime::now().checked_add(Duration::from_secs(7200)).unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;

        Self { signup_id, iat, exp, subj: "Signup".to_string(), }
    }
}

impl Jwt for SignupJwt {}


// persist on redis ??
#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordJwt {
    forgot_id: Uuid, // unique id generated when a user requests for password change, this should be persisted
    exp: usize,
    iat: usize,
    subj: String,
}

impl Jwt for ForgotPasswordJwt {}

impl ForgotPasswordJwt {
    pub fn new(forgot_id: Uuid) -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        let exp = SystemTime::now().checked_add(Duration::from_secs(1200)).unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        Self { forgot_id, exp, iat, subj: "Forgot".to_string(), }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LoginJwt {
    email: String,
    user_id: Uuid,
    exp: usize,
    iat: usize,
    subj: String,
}

impl LoginJwt {
    pub fn new(email: ValidEmail, user_id: Uuid) -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        let exp = SystemTime::now().checked_add(Duration::from_secs(1200)).unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize;
        Self { email: email.to_string(), exp, iat, subj: "Login".to_string(), user_id }
    }
}

impl Jwt for LoginJwt {}

#[cfg(test)]
mod test_jwt {
    use super::*;
    use crate::helpers::{test_helpers::get_appsettings, mail::Email};
    use fake::Fake;

    pub fn get_email() -> ValidEmail {
        let raw_email: String = fake::faker::internet::en::SafeEmail().fake();
        Email::parse(raw_email).unwrap()
    }

    #[test]
    fn generates_token_on_forgot() {
        let forgot_uuid: Uuid = Uuid::new_v4();
        let envs = get_appsettings();

        let token = ForgotPasswordJwt::new(forgot_uuid).encode(&envs);
        assert!(token.is_ok());
    }

    #[test]
    fn generated_forgot_token_can_be_decoded() {
        let forgot_uuid: Uuid = Uuid::new_v4();
        let envs = get_appsettings();

        let encoded_token = ForgotPasswordJwt::new(forgot_uuid).encode(&envs).unwrap();
        let decoded_token: Result<TokenData<ForgotPasswordJwt>, _> = ForgotPasswordJwt::decode(&encoded_token, &envs);

        assert!(decoded_token.is_ok());
        let token = decoded_token.unwrap();
        assert_eq!(token.claims.subj, "Forgot".to_string());
        assert_eq!(token.claims.forgot_id, forgot_uuid);
    }

    #[test]
    fn forgot_token_expires_after_two_hours() {
        let forgot_uuid: Uuid = Uuid::new_v4();
        const TWENTY_MINUTES: usize = 1200000; // Equivalent of twenty minutes in ms
        let envs = get_appsettings();

        let encoded_token = ForgotPasswordJwt::new(forgot_uuid).encode(&envs).unwrap();
        let decoded_token: TokenData<ForgotPasswordJwt> = ForgotPasswordJwt::decode(&encoded_token, &envs).unwrap();
        
        let active_period = decoded_token.claims.exp - decoded_token.claims.iat;
        assert_eq!(active_period, TWENTY_MINUTES);
        assert_eq!(decoded_token.claims.subj, "Forgot".to_string());

    }

    #[test]
    fn generates_token_on_login() {
        let login_uuid: Uuid = Uuid::new_v4();
        let envs = get_appsettings();
        let email = get_email();

        let token = LoginJwt::new(email, login_uuid).encode(&envs);
        assert!(token.is_ok());
    }

    #[test]
    fn generated_login_token_can_be_decoded() {
        
        let login_uuid: Uuid = Uuid::new_v4();
        let envs = get_appsettings();
        let email = get_email();

        let encoded_token = LoginJwt::new(email.clone(), login_uuid).encode(&envs).unwrap();
        let decoded_token: Result<TokenData<LoginJwt>, _> = LoginJwt::decode(&encoded_token, &envs);

        assert!(decoded_token.is_ok());
        let token = decoded_token.unwrap();
        assert_eq!(token.claims.subj, "Login".to_string());
        assert_eq!(token.claims.email, email.to_string());
    }

    #[test]
    fn login_token_expires_after_two_hours() {
        let login_uuid: Uuid = Uuid::new_v4();
        const TWENTY_MINUTES: usize = 1200000; // Equivalent of twenty minutes in ms
        let envs = get_appsettings();
        let email = get_email();

        let encoded_token = LoginJwt::new(email, login_uuid).encode(&envs).unwrap();
        let decoded_token: TokenData<LoginJwt> = LoginJwt::decode(&encoded_token, &envs).unwrap();
        
        let active_period = decoded_token.claims.exp - decoded_token.claims.iat;
        assert_eq!(active_period, TWENTY_MINUTES);
        assert_eq!(decoded_token.claims.subj, "Login".to_string());
    }

    #[test]
    fn generates_token_on_signup() {
        let signup_id: Uuid = Uuid::new_v4();
        let token = SignupJwt::new(signup_id).encode(&get_appsettings());
        assert!(token.is_ok());
    }

    #[test]
    fn generated_signup_token_can_be_decoded() {
        let signup_id: Uuid = Uuid::new_v4();
        let envs = get_appsettings();

        let encoded_token = SignupJwt::new(signup_id).encode(&envs).unwrap();
        let decoded_token: Result<TokenData<SignupJwt>, _> = SignupJwt::decode(&encoded_token, &envs);

        assert!(decoded_token.is_ok());
        assert_eq!(decoded_token.unwrap().claims.subj, "Signup".to_string());
    }

    #[test]
    fn signup_token_expires_after_two_hours() {
        let signup_id: Uuid = Uuid::new_v4();
        const TWO_HOURS: usize = 7200000; // Equivalent of two hours in ms
        let envs = get_appsettings();

        let encoded_token = SignupJwt::new(signup_id).encode(&envs).unwrap();
        let decoded_token: TokenData<SignupJwt> = SignupJwt::decode(&encoded_token, &envs).unwrap();
        
        let active_period = decoded_token.claims.exp - decoded_token.claims.iat;
        assert_eq!(active_period, TWO_HOURS);
    }
}
