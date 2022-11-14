use actix::prelude::Message;
use serde::Serialize;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct AlreadyQueued;

#[derive(Message, Clone, Serialize)]
#[rtype(result = "()")]
pub(crate) struct MatchMessage {
    pub msg: String,
    pub match_id: Uuid,
}
