use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub points: i64,
    pub locale: String,
    pub avatar: String,

    #[serde(rename = "type")]
    pub user_type: String,
    pub premium: i64,
    pub expiration: String,
}
