use crate::game::command::ClientCommand;
use crate::game::messages::{ClientCommandMessage, UserJoined, UserLeft};
use crate::game::Game;

use super::{message::*, ServerResponseMessage};
use actix::{Actor, Running, StreamHandler};
use actix::{ActorContext, Addr};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws::Message::Text;
use actix_web_actors::ws::{self, CloseCode, CloseReason};
use log::info;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct GameWebsocket {
    hb: Instant,
    user_id: i64,
    game: Addr<Game>,
}

impl GameWebsocket {
    pub fn new(user_id: i64, game: Addr<Game>) -> Self {
        Self {
            hb: Instant::now(),
            user_id,
            game,
        }
    }
}

impl Actor for GameWebsocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.game.do_send(UserJoined(self.user_id, ctx.address()));
        self.hb(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.game.do_send(UserLeft(self.user_id));
        Running::Stop
    }
}

impl GameWebsocket {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Disconnecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"hi");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameWebsocket {
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
                let cmd = ClientCommand::parse_command(s.to_string());
                if let ClientCommand::Error(s) = cmd {
                    ctx.text(s);
                } else {
                    self.game.do_send(ClientCommandMessage(self.user_id, cmd));
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}

impl Handler<WsMessage> for GameWebsocket {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl Handler<StopConnection> for GameWebsocket {
    type Result = ();

    fn handle(&mut self, _: StopConnection, ctx: &mut Self::Context) {
        ctx.close(Some(CloseReason::from(CloseCode::Error)));
        ctx.stop();
    }
}

impl Handler<ServerResponseMessage> for GameWebsocket {
    type Result = ();

    fn handle(&mut self, msg: ServerResponseMessage, ctx: &mut Self::Context) {
        let text = serde_json::to_string(&msg).unwrap();
        ctx.text(text);
        if msg.cmd == "result" {
            ctx.run_later(Duration::from_secs(3), |_, ctx| {
                ctx.close(Some(CloseReason::from(CloseCode::Error)));
                ctx.stop();
            });
        }
    }
}
