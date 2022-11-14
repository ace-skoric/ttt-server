use actix::Message;
use serde::Serialize;

use crate::game::game_state::{State, Timers, UserGameState, UserPlayer};

#[derive(Serialize)]
#[serde(tag = "cmd", content = "msg")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ServerResponse {
    OppUnhover(usize),
    OppHover(usize),
    YouPlay(usize),
    OppPlay(usize),
    TurnPlayer(UserPlayer),
    GameStatus(State),
    GameResult(String),
    GameState(UserGameState),
    Time(Timers),
    Error(String),
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub(crate) struct ServerResponseMessage(pub ServerResponse);
