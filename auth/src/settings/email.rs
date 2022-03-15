use serde::Deserialize;
use serde_with_expand_env::with_expand_envs;

#[derive(Debug, Deserialize, Clone)]
pub struct EmailSettings {
    #[serde(deserialize_with = "with_expand_envs")]
    pub smtp_user: String,
    #[serde(deserialize_with = "with_expand_envs")]
    pub smtp_pass: String,
    #[serde(deserialize_with = "with_expand_envs")]
    pub smtp_server: String,
}
