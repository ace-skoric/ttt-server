use crate::ttt_db::{TttDbConn, TttDbErr};
use crate::util::range::calculate_elo_range;
use crate::util::time::get_time_in_queue;
use chrono::Utc;
use redis::AsyncCommands;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct PlayerData {
    pub user_id: i64,
    pub username: String,
    pub elo: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct Match {
    pub match_id: Uuid,
    pub players: (PlayerData, PlayerData),
}

impl TttDbConn {
    pub async fn check_if_queued(&self, user_id: i64) -> Result<bool, TttDbErr> {
        let mut rdb = self.rdb.get_async_connection().await?;
        let res = rdb.zscore::<&str, i64, i64>("mm_pool", user_id).await;
        match res {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    pub async fn insert_user_into_mm_queue(&self, user_id: i64) -> Result<(), TttDbErr> {
        let mut rdb = self.rdb.get_async_connection().await?;
        if self.check_if_queued(user_id).await? {
            return Err(TttDbErr::UserAlreadyQueued);
        }
        let elo = self.get_elo(user_id).await?;
        rdb.zadd("mm_pool", user_id, elo).await?;
        let time = Utc::now().naive_utc().timestamp();
        rdb.zadd("mm_time", user_id, time).await?;
        Ok(())
    }
    pub async fn remove_user_from_mm_queue(&self, user_id: i64) -> Result<(), TttDbErr> {
        let mut rdb = self.rdb.get_async_connection().await?;
        rdb.zrem("mm_pool", user_id).await?;
        rdb.zrem("mm_time", user_id).await?;
        Ok(())
    }
    pub async fn create_match(&self, p1_id: i64, p2_id: i64) -> Result<Match, TttDbErr> {
        let p1 = self.find_user_by_id(p1_id).await?;
        let p2 = self.find_user_by_id(p2_id).await?;
        let p1_elo = self.get_elo(p1_id).await?;
        let p2_elo = self.get_elo(p2_id).await?;
        let p1 = PlayerData {
            user_id: p1.user_id,
            username: p1.username,
            elo: p1_elo,
        };
        let p2 = PlayerData {
            user_id: p2.user_id,
            username: p2.username,
            elo: p2_elo,
        };
        let match_id = Uuid::new_v4();
        let new_match = Match {
            match_id,
            players: (p1, p2),
        };
        let mut rdb = self.rdb.get_async_connection().await?;
        let match_message = serde_json::to_string(&new_match).unwrap();
        rdb.publish("matchmaking", match_message).await?;
        Ok(new_match)
    }
    pub async fn find_matches(&self) -> Result<Vec<Match>, TttDbErr> {
        let mut rdb = self.rdb.get_async_connection().await?;
        let mut rdb2 = self.rdb.get_async_connection().await?;
        let mut iter = rdb2.zscan::<&str, (i64, i64)>("mm_time").await?;
        let mut matches = Vec::<Match>::new();
        while let Some((user_id, time_joined)) = iter.next_item().await {
            let elo = rdb.zscore::<&str, i64, u64>("mm_pool", user_id).await;
            let elo = match elo {
                Ok(elo) => elo,
                Err(_) => continue,
            };
            let time = get_time_in_queue(time_joined);
            let elo_range = calculate_elo_range(time);
            let mut possible_opponents = Vec::<(i64, i64)>::new();
            let opponents: Vec<(i64, u64)> = rdb
                .zrangebyscore_withscores("mm_pool", elo - elo_range, elo + elo_range)
                .await?;
            for (opp_id, opp_elo) in opponents {
                if opp_id == user_id {
                    continue;
                }
                let opp_time: i64 = rdb.zscore("mm_time", opp_id).await?;
                let opp_time = get_time_in_queue(opp_time);
                let opp_range = calculate_elo_range(time);
                if opp_elo - opp_range <= elo && elo <= opp_elo + opp_range {
                    possible_opponents.push((opp_id, opp_time));
                }
            }
            if !possible_opponents.is_empty() {
                possible_opponents.sort_unstable_by(|a, b| b.1.cmp(&a.1));
                let opp_id = possible_opponents[0].0;
                self.remove_user_from_mm_queue(user_id).await?;
                self.remove_user_from_mm_queue(opp_id).await?;
                let new_match = self.create_match(user_id, opp_id).await?;
                matches.push(new_match);
            }
        }
        Ok(matches)
    }
}
