use fancy_regex::Regex;
use lazy_static::lazy_static;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, Transport};

use crate::errors::app::ApiError;
#[cfg(feature = "test")]
use crate::stubs::email::SmtpTransport;
#[cfg(not(feature = "test"))]
use lettre::SmtpTransport;

use crate::settings::email::EmailSettings;
use crate::helpers::email_template::{signup_template, forgot_template};

#[cfg(feature = "test")]
#[path = "./email.test.rs"]
mod email_test;

#[derive(Debug)]
pub struct MailInfo {
    token: String,
    frontend_url: String,
}

impl MailInfo {
    pub fn new(token: String, url: &String) -> Self {
        Self {token, frontend_url: url.to_string()}
    }

    pub fn token(&self) -> String {
        self.token.to_string()
    }

    pub fn url(&self) -> String {
        self.frontend_url.to_string()
    }
}

#[derive(Debug)]
pub enum MailType {
    Signup(MailInfo),
    ForgotPassword(MailInfo),
}

impl MailType {
    pub fn content(&self) -> String {        
        match self {
            Self::ForgotPassword(info) => {
                let url = format!("{}/{}", info.url(), info.token());
                forgot_template(url)
            }
            Self::Signup(info) => {
                let url = format!("{}/{}", info.url(), info.token());
                signup_template(url)
            }
        }
    }
}


pub struct Email {
    recipient_email: ValidEmail,
    recipient_name: Option<String>,
    email_type: MailType,
}

#[derive(Debug, Clone, PartialEq, derive_more::Display)]
pub struct ValidEmail(#[display(fmt = "{0}")]String);


impl Email {
    pub fn new(recipient_email: ValidEmail, recipient_name: Option<String>, email_type: MailType) -> Self {
        Self {
            recipient_email,
            recipient_name,
            email_type,
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
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(self.email_type.content())
                    ) 
            )
            // change this to ? to cascade the handle the appropriate response here
            .expect("failed to build email");

        let creds = Credentials::new(smtp_user.clone(), smtp_pass.clone());

        let mailer = SmtpTransport::relay(&smtp_server)
            .unwrap().credentials(creds).build();

        if let Err(e) = mailer.send(&email) {
            // persist these errors * tracing??
            println!("ERROR SENDING EMAIL?????? {:#?}", e);
        }
    }
}