use actix::prelude::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct StopConnection;
