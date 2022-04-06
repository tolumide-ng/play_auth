#[cfg(test)]
mod test_jwt {
    use crate::helpers::test_helpers::get_appsettings;
    use crate::helpers::mails::email::{Email, ValidEmail};
    use crate::helpers::jwt_tokens::jwt::{ForgotPasswordJwt, LoginJwt, SignupJwt, Jwt};
    use fake::Fake;
    use auth_macro::jwt::JwtHelper;
    use jsonwebtoken::TokenData;
    use uuid::Uuid;

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
        let user_id: Uuid = Uuid::new_v4();
        let envs = get_appsettings();

        let encoded_token = ForgotPasswordJwt::new(user_id).encode(&envs).unwrap();
        let decoded_token: Result<TokenData<ForgotPasswordJwt>, _> = ForgotPasswordJwt::decode(&encoded_token, &envs);

        assert!(decoded_token.is_ok());
        let token = decoded_token.unwrap();
        assert_eq!(token.claims.subj, "Forgot".to_string());
        assert_eq!(token.claims.get_user(), user_id);
    }

    #[test]
    fn forgot_token_expires_after_two_minutes() {
        let user_id: Uuid = Uuid::new_v4();
        const TWENTY_MINUTES: usize = 1200000; // Equivalent of twenty minutes in ms
        let envs = get_appsettings();

        let encoded_token = ForgotPasswordJwt::new(user_id).encode(&envs).unwrap();
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
        let context = "random_str".to_string();

        let token = LoginJwt::new(email, login_uuid, context,  true).encode(&envs);
        assert!(token.is_ok());
    }

    #[test]
    fn generated_login_token_can_be_decoded() {
        
        let login_uuid: Uuid = Uuid::new_v4();
        let envs = get_appsettings();
        let email = get_email();
        let context = "random_str".to_string();

        let encoded_token = LoginJwt::new(email.clone(), login_uuid, context, true).encode(&envs).unwrap();
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
        let context = "random_str".to_string();

        let encoded_token = LoginJwt::new(email, login_uuid, context, false).encode(&envs).unwrap();
        let decoded_token: TokenData<LoginJwt> = LoginJwt::decode(&encoded_token, &envs).unwrap();
        
        let active_period = decoded_token.claims.exp - decoded_token.claims.iat;
        assert_eq!(active_period, TWENTY_MINUTES);
        assert_eq!(decoded_token.claims.subj, "Login".to_string());
    }

    #[test]
    fn generates_token_on_signup() {
        let user_id: Uuid = Uuid::new_v4();
        let token = SignupJwt::new(user_id).encode(&get_appsettings());
        assert!(token.is_ok());
    }

    #[test]
    fn generated_signup_token_can_be_decoded() {
        let user_id: Uuid = Uuid::new_v4();
        let envs = get_appsettings();

        let encoded_token = SignupJwt::new(user_id).encode(&envs).unwrap();
        let decoded_token: Result<TokenData<SignupJwt>, _> = SignupJwt::decode(&encoded_token, &envs);

        assert!(decoded_token.is_ok());
        assert_eq!(decoded_token.unwrap().claims.subj, "Signup".to_string());
    }

    #[test]
    fn signup_token_expires_after_two_hours() {
        let user_id: Uuid = Uuid::new_v4();
        const TWO_HOURS: usize = 7200000; // Equivalent of two hours in ms
        let envs = get_appsettings();

        let encoded_token = SignupJwt::new(user_id).encode(&envs).unwrap();
        let decoded_token: TokenData<SignupJwt> = SignupJwt::decode(&encoded_token, &envs).unwrap();
        
        let active_period = decoded_token.claims.exp - decoded_token.claims.iat;
        assert_eq!(active_period, TWO_HOURS);
    }
}
