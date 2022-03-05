use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use secrecy::{Secret, ExposeSecret};

use crate::settings::variables::EnvVars;

pub enum MailType {
    // signup or forgot_pwd key/jwt
    Signup(&'static str),
    ForgotPassword(&'static str),
}

pub struct Email {
    recipient_email: String,
    recipient_name: String,
    email_type: MailType,
}


impl Email {
    pub fn new(recipient_email: String, recipient_name: String, email_type: MailType) -> Self {
        Self {
            recipient_email, 
            recipient_name,
            email_type,
        }
    }

    pub fn send_email(self) {
        // maybe just use Postfix
        let EnvVars { smtp_user, smtp_pass, smtp_server, .. } = EnvVars::new();
        let email = Message::builder()
            .from("Dire <noreply@gmail.com>".parse().unwrap())
            .to(format!("{} <{}>", self.recipient_name, self.recipient_email).parse().unwrap())
            .subject("Welcome to the app with no name yet!")
            // use html file styled with css in this case
            .body(String::from(r#"Thank you for signing up with us :wink, please 
            activate your account by clicking the link: <>whatever the links<>"#)).unwrap();

        let creds = Credentials::new(smtp_user, smtp_pass);

        let mailer = SmtpTransport::relay(&smtp_server)
            .unwrap().credentials(creds).build();

        if let Err(e) = mailer.send(&email) {
            // persist this errors
            println!("ERROR SENDING EMAIL?????? {:#?}", e);
        }
    }
}