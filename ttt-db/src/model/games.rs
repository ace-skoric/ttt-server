use crate::entity::{games, user_stats};
use crate::{TttDbConn, TttDbErr};
use redis::AsyncCommands;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::*, QueryFilter, TransactionTrait};
use skillratings::config::EloConfig;
use skillratings::elo::elo;
use skillratings::outcomes::Outcomes;
use skillratings::rating::EloRating;
use uuid::Uuid;

impl TttDbConn {
    pub async fn create_active_game(
        &self,
        game_id: Uuid,
        players: (i64, i64),
    ) -> Result<(), TttDbErr> {
        let mut rdb = self.rdb.get_async_connection().await?;
        rdb.hset_multiple(
            format!("active_game:{}", game_id),
            &[("player1_id", players.0), ("player2_id", players.1)],
        )
        .await?;
        Ok(())
    }
    pub async fn delete_active_game(&self, game_id: Uuid) -> Result<(), TttDbErr> {
        let mut rdb = self.rdb.get_async_connection().await?;
        rdb.del(format!("active_game:{}", game_id)).await?;
        Ok(())
    }
    pub async fn check_user_in_active_game(
        &self,
        game_id: Uuid,
        user_id: i64,
    ) -> Result<bool, TttDbErr> {
        let mut rdb = self.rdb.get_async_connection().await?;
        let res = rdb
            .hgetall::<String, Vec<(String, i64)>>(format!("active_game:{}", game_id))
            .await?;
        for user in res {
            if user_id == user.1 {
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub async fn record_game(
        &self,
        game_id: Uuid,
        user1_id: i64,
        user2_id: i64,
        winner: Option<i64>,
        start_time: DateTimeWithTimeZone,
        end_time: DateTimeWithTimeZone,
    ) -> Result<(), TttDbErr> {
        let db = &self.db;
        self.delete_active_game(game_id).await?;
        games::ActiveModel {
            game_id: Set(game_id),
            user1_id: Set(user1_id),
            user2_id: Set(user2_id),
            winner: Set(winner),
            start_time: Set(start_time),
            end_time: Set(end_time),
        }
        .insert(db)
        .await?;
        let p1_data = self.get_user_data(user1_id).await?;
        let p2_data = self.get_user_data(user2_id).await?;
        let p1_elo = EloRating {
            rating: p1_data.elo as f64,
        };
        let p2_elo = EloRating {
            rating: p2_data.elo as f64,
        };
        let outcome = match winner {
            Some(id) => {
                if id == user1_id {
                    Outcomes::WIN
                } else {
                    Outcomes::LOSS
                }
            }
            None => Outcomes::DRAW,
        };
        let config = EloConfig::new();
        let p1_games = match outcome {
            Outcomes::WIN => (p1_data.wins + 1, p1_data.draws, p1_data.losses),
            Outcomes::DRAW => (p1_data.wins, p1_data.draws + 1, p1_data.losses),
            Outcomes::LOSS => (p1_data.wins, p1_data.draws, p1_data.losses + 1),
        };
        let p2_games = match outcome {
            Outcomes::WIN => (p2_data.wins, p1_data.draws, p1_data.losses + 1),
            Outcomes::DRAW => (p2_data.wins, p1_data.draws + 1, p1_data.losses),
            Outcomes::LOSS => (p2_data.wins + 1, p1_data.draws, p1_data.losses),
        };
        let (p1_elo, p2_elo) = elo(&p1_elo, &p2_elo, &outcome, &config);
        let p1_elo = p1_elo.rating as i64;
        let p2_elo = p2_elo.rating as i64;
        let mut p1 = p1_data.into_active_model();
        let mut p2 = p2_data.into_active_model();
        p1.set(user_stats::Column::Elo, Value::BigInt(Some(p1_elo)));
        p1.set(user_stats::Column::Wins, Value::BigInt(Some(p1_games.0)));
        p1.set(user_stats::Column::Draws, Value::BigInt(Some(p1_games.1)));
        p1.set(user_stats::Column::Losses, Value::BigInt(Some(p1_games.2)));
        p1.update(db).await?;
        p2.set(user_stats::Column::Elo, Value::BigInt(Some(p2_elo)));
        p2.set(user_stats::Column::Wins, Value::BigInt(Some(p2_games.0)));
        p2.set(user_stats::Column::Draws, Value::BigInt(Some(p2_games.1)));
        p2.set(user_stats::Column::Losses, Value::BigInt(Some(p2_games.2)));
        p2.update(db).await?;
        Ok(())
    }
}
