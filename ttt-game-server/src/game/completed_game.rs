use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CompletedGame {
    pub game_id: Uuid,
    pub player1_id: i64,
    pub player1_elo: i64,
    pub player2_id: i64,
    pub player2_elo: i64,
    pub winner: Option<i64>,
    pub game_start_time: DateTime<Utc>,
    pub game_end_time: DateTime<Utc>,
}
