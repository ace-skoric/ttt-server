use actix::{Addr, Message};

use crate::ws::ws::MatchmakingWebsocket;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct FindMatches;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub(crate) struct AddUserToQueue(pub i64, pub Addr<MatchmakingWebsocket>);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub(crate) struct RemoveUserFromQueue(pub i64);
