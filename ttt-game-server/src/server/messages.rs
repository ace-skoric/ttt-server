use actix::{Addr, Message};
use uuid::Uuid;

use ttt_db::Match;

use crate::game::{completed_game::CompletedGame, Game};

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateNewGame(pub Match);

#[derive(Message)]
#[rtype(result = "Option<Addr<Game>>")]
pub struct GetGameAddress(pub Uuid);

#[derive(Message)]
#[rtype(result = "()")]
pub struct GameEnded(pub CompletedGame);
