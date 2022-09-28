use actix::{fut::wrap_future, Actor, ActorFutureExt, Addr, AsyncContext, Context, Handler};
use log::{info, warn};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use ttt_db::TttDbConn;

use crate::game::Game;

use super::messages::*;

#[derive(Debug, Clone)]
pub struct GameServer {
    db: Arc<TttDbConn>,
    games: HashMap<Uuid, Addr<Game>>,
}

impl GameServer {
    pub fn new(db: Arc<TttDbConn>) -> Self {
        Self {
            db,
            games: HashMap::new(),
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("Game Server is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("Game Server stopped");
    }
}

impl Handler<CreateNewGame> for GameServer {
    type Result = ();
    fn handle(&mut self, msg: CreateNewGame, ctx: &mut Self::Context) -> Self::Result {
        let m = msg.0;
        let db = self.db.clone();
        let create_active = wrap_future::<_, Self>(async move {
            db.create_active_game(m.match_id, (m.players.0.user_id, m.players.1.user_id))
                .await
        });
        let create_active = create_active.map(|_, _, _| ());
        ctx.spawn(create_active);
        let game_id = m.match_id;
        let game = Game::new(m, ctx.address()).start();
        self.games.insert(game_id, game);
    }
}

impl Handler<GetGameAddress> for GameServer {
    type Result = Option<Addr<Game>>;
    fn handle(&mut self, msg: GetGameAddress, _: &mut Self::Context) -> Self::Result {
        let id = msg.0;
        let res = self.games.get(&id);
        match res {
            Some(addr) => Some(addr.clone()),
            None => None,
        }
    }
}

impl Handler<GameEnded> for GameServer {
    type Result = ();
    fn handle(&mut self, msg: GameEnded, ctx: &mut Self::Context) -> Self::Result {
        let game = msg.0;
        self.games.remove(&game.game_id);
        let db = self.db.clone();
        let fut = wrap_future::<_, Self>(async move {
            db.record_game(
                game.game_id,
                game.player1_id,
                game.player2_id,
                game.winner,
                game.game_start_time.into(),
                game.game_end_time.into(),
            )
            .await
        });
        let fut = fut.map(|res, _, _| {
            if let Err(err) = res {
                warn!("{:?}", err);
            }
        });
        ctx.spawn(fut);
    }
}
