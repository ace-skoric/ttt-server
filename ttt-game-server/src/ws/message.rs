use actix::prelude::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct StopConnection;
