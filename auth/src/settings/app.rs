use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub port: String,
    pub env: String,
    pub m_cost: u16,
    pub p_cost: u16,
    pub t_cost: u16,
    pub jwt_secret: String,
    pub frontend_url: String,
}