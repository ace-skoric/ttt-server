use actix::Message;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendVerificationEmail {
    pub username: String,
    pub email: String,
    pub uuid: Uuid,
}

impl SendVerificationEmail {
    pub fn new(username: String, email: String, uuid: Uuid) -> Self {
        Self {
            username,
            email,
            uuid,
        }
    }
}
