use serde::Deserialize;
use serde_with_expand_env::with_expand_envs;

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    #[serde(deserialize_with = "with_expand_envs")]
    pub port: String,
    #[serde(deserialize_with = "with_expand_envs")]
    pub env: String,
    #[serde(deserialize_with = "with_expand_envs")]
    pub m_cost: u16,
    #[serde(deserialize_with = "with_expand_envs")]
    pub p_cost: u16,
    #[serde(deserialize_with = "with_expand_envs")]
    pub t_cost: u16,
    #[serde(deserialize_with = "with_expand_envs")]
    pub jwt_secret: String,
}