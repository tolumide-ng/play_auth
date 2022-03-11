use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with_expand_env::with_expand_envs;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub port: String,
    pub env: String,
    #[serde(deserialize_with = "with_expand_envs")]
    pub m_cost: u16,
    #[serde(deserialize_with = "with_expand_envs")]
    pub p_cost: u16,
    #[serde(deserialize_with = "with_expand_envs")]
    pub t_cost: u16,
    pub jwt_secret: String,
}