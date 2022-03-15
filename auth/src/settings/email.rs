use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct EmailSettings {
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_server: String,
}
