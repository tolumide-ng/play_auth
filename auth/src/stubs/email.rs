pub struct SmtpTransport;
pub struct SmtpTransportBuilder;
pub struct Transport;


use lettre::{transport::smtp::authentication::Credentials, Message};

impl SmtpTransport {
    pub fn relay(relay: &str) -> Result<SmtpTransportBuilder, ()> {
        Ok(SmtpTransportBuilder)
    }

    pub fn send(&self, _: &Message) -> Result<(), ()> {
        Ok(())
    }
}


impl SmtpTransportBuilder {
    pub fn credentials(self, _: Credentials) -> Self {
        self
    }

    pub fn build(self) -> SmtpTransport {
        SmtpTransport
    }
}

