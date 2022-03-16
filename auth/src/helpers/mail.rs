use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, Transport};


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

pub struct Email {
    recipient_email: String,
    recipient_name: Option<String>,
    email_type: MailType,
}


impl Email {
    pub fn new(recipient_email: String, recipient_name: Option<String>, email_type: MailType) -> Self {
        Self {
            recipient_email, 
            recipient_name,
            email_type,
        }
    }

    pub fn send_email(self, mail: &EmailSettings) {
        // maybe just use Postfix
        let EmailSettings { smtp_user, smtp_pass, smtp_server } = mail;
        let mut person_name = self.recipient_email.clone();

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