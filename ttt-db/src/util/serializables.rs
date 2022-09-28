use serde::{Deserialize, Serialize};

pub use crate::entity::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMessage {
    pub email: String,
    #[serde(default)]
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataMessage {
    pub username: String,
    pub elo: i32,
    pub wins: i64,
    pub losses: i64,
    pub draws: i64,
}
