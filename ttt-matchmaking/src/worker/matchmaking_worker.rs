use actix::{fut::wrap_future, Actor, ActorFutureExt, Addr, AsyncContext, Context, Handler};
use log::{error, info, warn};
use std::{collections::HashMap, sync::Arc, time::Duration};
use ttt_game_server::server::messages::CreateNewGame;
use ttt_game_server::server::GameServer;

use ttt_db::{Match, TttDbConn};

use crate::ws::{message::MatchMessage, MatchmakingWebsocket};

use super::messages::*;

#[derive(Debug, Clone)]
pub struct MatchmakingWorker {
    db: Arc<TttDbConn>,
    active_users: HashMap<i64, Addr<MatchmakingWebsocket>>,
    game_server: Addr<GameServer>,
}

impl MatchmakingWorker {
    pub fn new(db: Arc<TttDbConn>, game_server: Addr<GameServer>) -> Self {
        Self {
            db,
            active_users: HashMap::new(),
            game_server,
        }
    }
    fn new_match(&self, new_match: Match) {
        let uuid = new_match.match_id.to_string();
        let msg = MatchMessage(uuid);
        let user_ids = (new_match.players.0.user_id, new_match.players.1.user_id);
        self.game_server.do_send(CreateNewGame(new_match));
        let addrs = (
            self.active_users.get(&user_ids.0),
            self.active_users.get(&user_ids.1),
        );
        if let Some(addr) = addrs.0 {
            addr.do_send(msg.clone());
        }
        if let Some(addr) = addrs.1 {
            addr.do_send(msg.clone());
        }
    }
}

impl Actor for MatchmakingWorker {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        info!("Matchmaking worker is alive");
        ctx.run_interval(Duration::from_secs(5), move |_this, ctx| {
            if !ctx.waiting() {
                ctx.notify(FindMatches {});
            }
        });
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("Matchmaking worker is stopped");
    }
}

impl Handler<FindMatches> for MatchmakingWorker {
    type Result = ();
    fn handle(&mut self, _msg: FindMatches, ctx: &mut Self::Context) -> Self::Result {
        let db = self.db.clone();
        let find_matches = wrap_future::<_, Self>(async move { db.find_matches().await });
        let find_matches = find_matches.map(|res, this, _ctx| match res {
            Ok(res) => {
                for m in res {
                    this.new_match(m);
                }
            }
            Err(err) => error!("Matchmaking error: {:?}!", err),
        });
        ctx.wait(find_matches);
    }
}

impl Handler<AddUserToQueue> for MatchmakingWorker {
    type Result = ();
    fn handle(&mut self, msg: AddUserToQueue, ctx: &mut Self::Context) -> Self::Result {
        let (user_id, addr) = (msg.0, msg.1);
        self.active_users.insert(user_id, addr);
        let db = self.db.clone();
        let add_user =
            wrap_future::<_, Self>(async move { db.insert_user_into_mm_queue(user_id).await });
        let add_user = add_user.map(|res, _this, _ctx| match res {
            Ok(_) => (),
            Err(err) => error!("Matchmaking error: {:?}!", err),
        });
        ctx.spawn(add_user);
        ctx.notify(FindMatches {});
    }
}

impl Handler<RemoveUserFromQueue> for MatchmakingWorker {
    type Result = ();
    fn handle(&mut self, msg: RemoveUserFromQueue, ctx: &mut Self::Context) -> Self::Result {
        let user_id = msg.0;
        self.active_users.remove(&user_id);
        let db = self.db.clone();
        let remove_user =
            wrap_future::<_, Self>(async move { db.remove_user_from_mm_queue(user_id).await });
        let remove_user = remove_user.map(|res, _this, _ctx| match res {
            Ok(_) => (),
            Err(err) => warn!("Matchmaking error: {:?}!", err),
        });
        ctx.spawn(remove_user);
    }
}
