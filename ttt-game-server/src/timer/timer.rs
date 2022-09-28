use actix::{Actor, Addr, AsyncContext, Context, Handler, SpawnHandle};
use std::time::Duration;

use super::messages::*;
use crate::game::messages::TimeExpired;
use crate::game::Game;

#[derive(Debug)]
pub struct Timer {
    player_id: i64,
    time: f32,
    game: Addr<Game>,
    handle: Option<SpawnHandle>,
}

impl Timer {
    pub fn new(player_id: i64, time: f32, game: Addr<Game>) -> Self {
        Self {
            player_id,
            time,
            game,
            handle: None,
        }
    }
}

impl Actor for Timer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {}

    fn stopped(&mut self, _ctx: &mut Context<Self>) {}
}

impl Handler<StartTimer> for Timer {
    type Result = ();

    fn handle(&mut self, _: StartTimer, ctx: &mut Self::Context) -> Self::Result {
        if let Some(handle) = self.handle {
            ctx.cancel_future(handle);
            self.handle = None;
        }
        self.handle = Some(ctx.run_interval(Duration::from_millis(50), |this, ctx| {
            this.time -= 0.05;
            if this.time <= 0.0 {
                ctx.notify(PauseTimer);
                this.time = 0.0;
                this.game.do_send(TimeExpired(this.player_id));
            }
        }));
    }
}

impl Handler<PauseTimer> for Timer {
    type Result = ();

    fn handle(&mut self, _: PauseTimer, ctx: &mut Self::Context) -> Self::Result {
        if let Some(handle) = self.handle {
            ctx.cancel_future(handle);
            self.handle = None;
        }
    }
}

impl Handler<GetTimer> for Timer {
    type Result = f32;
    fn handle(&mut self, _: GetTimer, _: &mut Self::Context) -> Self::Result {
        self.time
    }
}

impl Handler<StopTimer> for Timer {
    type Result = ();

    fn handle(&mut self, _msg: StopTimer, _ctx: &mut Self::Context) -> Self::Result {}
}
