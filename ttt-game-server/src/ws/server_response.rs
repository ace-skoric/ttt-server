use actix::Message;
use serde::Serialize;

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub(crate) struct ServerResponseMessage {
    pub cmd: String,
    pub msg: String,
}

impl ServerResponseMessage {
    pub(crate) fn new(cmd: &str, msg: &str) -> Self {
        Self {
            cmd: cmd.to_string(),
            msg: msg.to_string(),
        }
    }
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub(crate) struct TimerMessage {
    pub your_time: f64,
    pub opp_time: f64,
}

impl TimerMessage {
    pub(crate) fn new(your_time: f64, opp_time: f64) -> Self {
        Self {
            your_time,
            opp_time,
        }
    }
}
