use actix_session::Session;
use serde::{Deserialize, Serialize};

use crate::util::error::TttApiErr;

#[derive(Serialize, Deserialize)]
pub struct UserSession {
    #[serde(skip_serializing)]
    pub id: i64,
    pub username: String,
    pub admin: bool,
    pub guest: bool,
}

pub trait SessionData {
    fn get_data(&self) -> Result<UserSession, TttApiErr>;
}

impl SessionData for Session {
    fn get_data(&self) -> Result<UserSession, TttApiErr> {
        let id = match self.get::<i64>("id") {
            Ok(Some(id)) => Ok(id),
            _ => Err(TttApiErr::forbidden()),
        }?;
        let admin = match self.get::<bool>("admin") {
            Ok(Some(admin)) => Ok(admin),
            _ => Err(TttApiErr::forbidden()),
        }?;
        let guest = match self.get::<bool>("guest") {
            Ok(Some(guest)) => Ok(guest),
            _ => Err(TttApiErr::forbidden()),
        }?;
        let username = match self.get::<String>("username") {
            Ok(Some(admin)) => Ok(admin),
            _ => Err(TttApiErr::forbidden()),
        }?;
        Ok(UserSession {
            id,
            admin,
            username,
            guest,
        })
    }
}
