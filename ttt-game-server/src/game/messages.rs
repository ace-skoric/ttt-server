use actix::{Addr, Message};

use crate::ws::GameWebsocket;

use super::ClientCommand;

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserJoined(pub i64, pub Addr<GameWebsocket>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserLeft(pub i64);

#[derive(Message)]
#[rtype(result = "()")]
pub struct TimeExpired(pub i64);

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct ClientCommandMessage(pub i64, pub ClientCommand);

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct EndgameMessage;
