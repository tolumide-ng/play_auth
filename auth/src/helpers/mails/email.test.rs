
#[cfg(test)]
#[cfg( feature = "test" )]
mod email_test {
    use fake::Fake;

    use crate::{helpers::mails::email::{MailInfo, MailType, Email}, settings::email::EmailSettings};


    #[test]
    fn creates_mailinfo_struct() {
        let token = "token".to_string();
        let url = "https://samplefrontend.com".to_string();

        let mail = MailInfo::new(token.clone(), &url);
        assert_eq!(mail.token(), token);
        assert_eq!(mail.url(), url);
    }


    #[test]
    fn signup_mailtype() {
        let token = "token".to_string();
        let url = "https://samplefrontend.com".to_string();

        let mail = MailInfo::new(token.clone(), &url);
        let signup_mail = MailType::Signup(mail);
        assert!(signup_mail.content().contains(&url));
        assert!(signup_mail.content().contains(&token));
        assert!(signup_mail.content().contains("Activate your account"));
        let uri = format!("{}/{}", url, token);
        assert!(signup_mail.content().contains(&uri));
    }


    #[test]
    fn forgotpassword_mailtype() {
        let token = "token".to_string();
        let url = "https://samplefrontend.com".to_string();

        let mail = MailInfo::new(token.clone(), &url);
        let forgot_mail = MailType::ForgotPassword(mail);
        assert!(forgot_mail.content().contains(&url));
        assert!(forgot_mail.content().contains(&token));
        assert!(forgot_mail.content().contains("Please click on the button below to reset password, this link expires in 20 minutes"));
        let uri = format!("{}/{}", url, token);
        assert!(forgot_mail.content().contains(&uri));
    }


    #[test]
    fn sends_a_signup_email() {
        let token = "token".to_string();
        let url = "https://samplefrontend.com".to_string();
        let email: String = fake::faker::internet::en::SafeEmail().fake();

        assert!(Email::parse(email.clone()).is_ok());

        let email = Email::parse(email.clone()).unwrap();

        let name: String = fake::faker::name::en::FirstName().fake();
        let mail = MailInfo::new(token, &url);
        let signup_mail = MailType::Signup(mail);

        let email_settings = EmailSettings {
            smtp_user: "smaplingperson".to_string(),
            smtp_pass: "simplepasssowrd".to_string(),
            smtp_server: "simpleserver".to_string(),
        };

        let email_maker = Email::new(email.clone(), Some(name), signup_mail);
        assert_eq!(email_maker.recipient_email, email);
        assert!(email_maker.recipient_name.is_some());
    }
}
