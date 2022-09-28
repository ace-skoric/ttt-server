use sea_orm::EntityTrait;

use crate::ttt_db::{TttDbConn, TttDbErr};

use crate::entity::prelude::UserStats;
use crate::entity::user_stats;

impl TttDbConn {
    pub async fn get_user_data(&self, user_id: i64) -> Result<UserStats, TttDbErr> {
        let db = &self.db;
        let res = user_stats::Entity::find_by_id(user_id).one(db).await?;
        match res {
            None => Err(TttDbErr::UserNotFound),
            Some(res) => Ok(res),
        }
    }
    pub async fn get_elo(&self, user_id: i64) -> Result<i64, TttDbErr> {
        let db = &self.db;
        let res = user_stats::Entity::find_by_id(user_id).one(db).await?;
        match res {
            None => Err(TttDbErr::UserNotFound),
            Some(res) => Ok(res.elo),
        }
    }
}
