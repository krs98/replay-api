use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCode(String);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, sqlx::Type)]
#[sqlx(type_name = "oauth_provider", rename_all = "lowercase")]
pub enum OAuthProvider {
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "bitbucket")]
    Bitbucket
}

