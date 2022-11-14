use crate::MatchmakingWorker;
use log::{debug, info};

use super::message::{AlreadyQueued, MatchMessage};
use crate::worker::messages::{AddUserToQueue, RemoveUserFromQueue};
use actix::{Actor, Running, StreamHandler};
use actix::{ActorContext, Addr};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws::Message::Text;
use actix_web_actors::ws::{self, CloseCode, CloseReason};
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct MatchmakingWebsocket {
    hb: Instant,
    user_id: i64,
    mm_worker: Addr<MatchmakingWorker>,
}

impl MatchmakingWebsocket {
    pub fn new(user_id: i64, mm_worker: Addr<MatchmakingWorker>) -> Self {
        Self {
            hb: Instant::now(),
            user_id,
            mm_worker,
        }
    }
}

impl Actor for MatchmakingWebsocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("User {} entered matchmaking", self.user_id);
        let msg = AddUserToQueue(self.user_id, ctx.address());
        self.mm_worker.do_send(msg);
        self.hb(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        let msg = RemoveUserFromQueue(self.user_id);
        self.mm_worker.do_send(msg);
        Running::Stop
    }
}

impl MatchmakingWebsocket {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                debug!("Disconnecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"hi");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MatchmakingWebsocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => {
                let message = s.parse::<u16>();
                if let Ok(0) = message {
                    ctx.close(Some(CloseReason::from(CloseCode::Normal)));
                };
            }
            Err(e) => panic!("{}", e),
        }
    }
}

impl Handler<AlreadyQueued> for MatchmakingWebsocket {
    type Result = ();

    fn handle(&mut self, _: AlreadyQueued, ctx: &mut Self::Context) {
        ctx.close(Some(CloseReason::from(CloseCode::Error)));
        ctx.stop();
    }
}

impl Handler<MatchMessage> for MatchmakingWebsocket {
    type Result = ();

    fn handle(&mut self, msg: MatchMessage, ctx: &mut Self::Context) {
        let msg = serde_json::to_string(&msg).unwrap();
        ctx.text(msg);
        ctx.run_later(Duration::from_secs(1), |_, ctx| {
            ctx.close(Some(CloseReason::from(CloseCode::Normal)));
            ctx.stop();
        });
    }
}
