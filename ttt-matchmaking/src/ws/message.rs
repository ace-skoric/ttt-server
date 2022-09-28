use actix::prelude::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct AlreadyQueued;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub(crate) struct MatchMessage(pub String);
