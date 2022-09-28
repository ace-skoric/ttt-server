use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartTimer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct PauseTimer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct StopTimer;

#[derive(Message)]
#[rtype(result = "f32")]
pub struct GetTimer;
