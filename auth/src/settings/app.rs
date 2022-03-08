use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub port: String,
    pub env: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub m_cost: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub p_cost: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub t_cost: u32,
    pub jwt_secret: String,
}