use fancy_regex::Regex;
use lazy_static::lazy_static;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, Transport};
use std::fmt;


use crate::errors::app::ApiError;
#[cfg(feature = "test")]
use crate::stubs::email::SmtpTransport;
#[cfg(not(feature = "test"))]
use lettre::SmtpTransport;

use crate::settings::email::EmailSettings;
pub enum MailType {
    // signup or forgot_pwd key/jwt
    Signup(&'static str),
    ForgotPassword(&'static str),
}

// CONVERT THIS EMAIL STRUCT INTO A TRAIT OBJECT
pub struct Email {
    recipient_email: ValidEmail,
    recipient_name: Option<String>,
    email_type: MailType,
    content: Option<String>,
}

#[derive(Debug, Clone, derive_more::Display)]
pub struct ValidEmail(#[display(fmt = "{0}")]String);


impl Email {
    pub fn new(recipient_email: ValidEmail, recipient_name: Option<String>, email_type: MailType, content: Option<String>) -> Self {
        Self {
            recipient_email,
            recipient_name,
            email_type,
            content,
        }
    }

    pub fn parse(email: String) -> Result<ValidEmail, ApiError> {
        lazy_static! {
            static ref USER_EMAIL: Regex = Regex::new(r#"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"#).unwrap();
        }

        if USER_EMAIL.is_match(&email).unwrap() {
            return Ok(ValidEmail(email))
        }

        Err(ApiError::ValidationError("Please provide a valid email address"))
    }

    pub fn send_email(self, mail: &EmailSettings) {
        // maybe just use Postfix
        let EmailSettings { smtp_user, smtp_pass, smtp_server } = mail;
        let mut person_name = self.recipient_email.clone().to_string();

        if let Some(name) = self.recipient_name {
            person_name = name;
        }
        let email = Message::builder()
            .from("Dire <noreply@gmail.com>".parse().unwrap())
            .to(format!("{} <{}>", person_name, self.recipient_email).parse().unwrap())
            .subject("Welcome to the app with no name yet!")
            // use html file styled with css in this case
            .body(String::from(r#"Thank you for signing up with us :wink, please 
            activate your account by clicking the link: <>whatever the links<>"#)).unwrap();

        let creds = Credentials::new(smtp_user.clone(), smtp_pass.clone());

        let mailer = SmtpTransport::relay(&smtp_server)
            .unwrap().credentials(creds).build();

        if let Err(e) = mailer.send(&email) {
            // persist this errors
            println!("ERROR SENDING EMAIL?????? {:#?}", e);
        }
    }
}