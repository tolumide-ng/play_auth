use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub port: String,
    pub env: String,
    pub m_cost: String,
    pub p_cost: String,
    pub t_cost: String,
    pub jwt_secret: String,
}