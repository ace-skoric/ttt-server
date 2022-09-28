use actix::{
    fut::wrap_future, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Context, Handler,
};
use chrono::{DateTime, Utc};
use log::info;
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;

use ttt_db::Match;

use crate::{
    game::game_state::State,
    server::{messages::GameEnded, GameServer},
    timer::{GetTimer, PauseTimer, StartTimer, StopTimer, Timer},
    ws::{GameWebsocket, ServerResponseMessage},
};

use super::{
    completed_game::CompletedGame,
    game_state::{GameState, Timers},
    messages::*,
    ClientCommand,
};

#[derive(Debug)]
pub struct Game {
    id: Uuid,
    game_state: GameState,
    srv: Addr<GameServer>,
    addrs: HashMap<i64, Addr<GameWebsocket>>,
    timers: HashMap<i64, Addr<Timer>>,
    started_at: DateTime<Utc>,
}

impl Game {
    pub fn new(game: Match, srv: Addr<GameServer>) -> Self {
        let id = game.match_id;
        let game_state = GameState::new(game.players);
        Self {
            id,
            game_state,
            srv,
            addrs: HashMap::new(),
            timers: HashMap::new(),
            started_at: Utc::now(),
        }
    }
}

impl Actor for Game {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        info!("Game {} created", self.id);
        let player_id = self.game_state.x_data.user_id;
        let first_turn = player_id;
        let timer = Timer::new(player_id, 60.0, ctx.address().clone()).start();
        self.timers.insert(player_id, timer);
        let player_id = self.game_state.o_data.user_id;
        let timer = Timer::new(player_id, 60.0, ctx.address().clone()).start();
        self.timers.insert(player_id, timer);
        ctx.run_later(Duration::from_secs(3), move |this, _| {
            this.game_state.state = State::Starting;
            let mut iter = this.addrs.iter();
            while let Some((_, addr)) = iter.next() {
                addr.do_send(ServerResponseMessage::new("starting", "Game starting soon"));
            }
        });
        ctx.run_later(Duration::from_secs(3), move |this, _| {
            this.game_state.state = State::Running;
            this.game_state.turn_player = Some(first_turn);
            this.timers.get(&first_turn).unwrap().do_send(StartTimer);
            let mut iter = this.addrs.iter();
            while let Some((user_id, addr)) = iter.next() {
                let game_state = this.game_state.to_msg(*user_id).serialize();
                addr.do_send(ServerResponseMessage::new("started", "Game started"));
                addr.do_send(ServerResponseMessage::new("game_state", &game_state));
            }
            info!("Game {} started", this.id);
        });
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("Game {} ended", self.id);
    }
}

impl Handler<UserJoined> for Game {
    type Result = ();

    fn handle(&mut self, msg: UserJoined, _: &mut Self::Context) -> Self::Result {
        let (user_id, addr) = (msg.0, msg.1);
        self.addrs.insert(user_id, addr.clone());
        let game_state = self.game_state.to_msg(user_id).serialize();
        addr.do_send(ServerResponseMessage::new("game_state", &game_state));
        info!("User {} joined game {}", user_id, self.id);
    }
}

impl Handler<UserLeft> for Game {
    type Result = ();

    fn handle(&mut self, msg: UserLeft, _: &mut Self::Context) -> Self::Result {
        let user_id = msg.0;
        self.addrs.remove(&user_id);
        info!("User {} left game {}", user_id, self.id);
    }
}

impl Handler<TimeExpired> for Game {
    type Result = ();

    fn handle(&mut self, msg: TimeExpired, ctx: &mut Self::Context) -> Self::Result {
        let loser_id = msg.0;
        let winner_id = if self.game_state.x_data.user_id == loser_id {
            self.game_state.o_data.user_id
        } else {
            self.game_state.x_data.user_id
        };
        self.game_state.winner = Some(winner_id);
        ctx.notify(EndgameMessage {});
    }
}

impl Handler<ClientCommandMessage> for Game {
    type Result = ();

    fn handle(&mut self, msg: ClientCommandMessage, ctx: &mut Self::Context) -> Self::Result {
        let (user_id, cmd) = (msg.0, msg.1);
        let player_id = user_id;
        let opp_data = if self.game_state.x_data.user_id == player_id {
            self.game_state.o_data.clone()
        } else {
            self.game_state.x_data.clone()
        };
        let opp_id = opp_data.user_id;
        let player_addr = self.addrs.get(&player_id);
        let opp_addr = self.addrs.get(&opp_id);
        match cmd {
            ClientCommand::Play(i) => match self.game_state.state {
                State::Created | State::Starting => {
                    let msg = ServerResponseMessage::new("error", "Game not started yet");
                    if let Some(addr) = player_addr {
                        addr.do_send(msg);
                    }
                    return;
                }
                State::Ended => {
                    let msg = ServerResponseMessage::new("error", "Game ended");
                    if let Some(addr) = player_addr {
                        addr.do_send(msg);
                    }
                    return;
                }
                State::Running => {
                    if self.game_state.turn_player.is_none() {
                        let msg = ServerResponseMessage::new("error", "Not your turn");
                        if let Some(addr) = player_addr {
                            addr.do_send(msg);
                        }
                        return;
                    } else if self.game_state.turn_player.unwrap() != user_id {
                        let msg = ServerResponseMessage::new("error", "Not your turn");
                        if let Some(addr) = player_addr {
                            addr.do_send(msg);
                        }
                        return;
                    } else {
                        if self.game_state.play(user_id, i) {
                            self.game_state.turn_player = Some(opp_id);
                            let player_timer = self.timers.get(&player_id).unwrap();
                            let opp_timer = self.timers.get(&opp_id).unwrap();
                            player_timer.do_send(PauseTimer {});
                            opp_timer.do_send(StartTimer {});
                            let p_message = ServerResponseMessage::new("you_play", &i.to_string());
                            let opp_message =
                                ServerResponseMessage::new("opp_play", &i.to_string());
                            let p_turn_player = self.game_state.to_msg(player_id).turn_player;
                            let p_turn_player = serde_json::to_string(&p_turn_player).unwrap();
                            let opp_turn_player = self.game_state.to_msg(opp_id).turn_player;
                            let opp_turn_player = serde_json::to_string(&opp_turn_player).unwrap();
                            if let Some(addr) = player_addr {
                                addr.do_send(p_message);
                                addr.do_send(ServerResponseMessage::new(
                                    "turn_player",
                                    &p_turn_player,
                                ));
                            }
                            if let Some(addr) = opp_addr {
                                addr.do_send(opp_message);
                                addr.do_send(ServerResponseMessage::new(
                                    "turn_player",
                                    &opp_turn_player,
                                ));
                            }
                            if self.game_state.check_endgame() {
                                ctx.notify(EndgameMessage {});
                            }
                            return;
                        } else {
                            let msg = ServerResponseMessage::new("error", "Not your turn");
                            if let Some(addr) = player_addr {
                                addr.do_send(msg);
                            }
                            return;
                        }
                    }
                }
            },
            ClientCommand::Hover(i) => {
                if let Some(addr) = opp_addr {
                    addr.do_send(ServerResponseMessage::new("opp_hover", &i.to_string()));
                }
            }
            ClientCommand::Unhover(i) => {
                if let Some(addr) = opp_addr {
                    addr.do_send(ServerResponseMessage::new("opp_unhover", &i.to_string()));
                }
            }
            ClientCommand::GetTurnPlayer => {
                if let Some(addr) = player_addr {
                    let turn_player = self.game_state.to_msg(player_id).turn_player;
                    let turn_player = serde_json::to_string(&turn_player).unwrap();
                    addr.do_send(ServerResponseMessage::new("turn_player", &turn_player));
                }
            }
            ClientCommand::GetGameState => {
                if let Some(addr) = player_addr {
                    let game_state = self.game_state.to_msg(player_id).serialize();
                    addr.do_send(ServerResponseMessage::new("game_state", &game_state));
                }
            }
            ClientCommand::GetTimers => {
                let opp_id = if self.game_state.x_data.user_id == user_id {
                    self.game_state.o_data.user_id
                } else {
                    self.game_state.x_data.user_id
                };
                let player_timer = self.timers.get(&user_id).unwrap().clone();
                let opp_timer = self.timers.get(&opp_id).unwrap().clone();
                if let Some(addr) = self.addrs.get(&user_id) {
                    let addr = addr.clone();
                    let time = wrap_future::<_, Self>(async move {
                        (
                            addr,
                            Timers {
                                you: player_timer.send(GetTimer).await.unwrap(),
                                opp: opp_timer.send(GetTimer).await.unwrap(),
                            },
                        )
                    });
                    let time = time.map(|res, _, _| {
                        res.0.do_send(ServerResponseMessage::new(
                            "timers",
                            &serde_json::to_string(&res.1).unwrap(),
                        ))
                    });
                    ctx.spawn(time);
                }
            }
            ClientCommand::Resign => {
                let loser_id = user_id;
                let winner_id = if self.game_state.x_data.user_id == loser_id {
                    self.game_state.o_data.user_id
                } else {
                    self.game_state.x_data.user_id
                };
                self.game_state.winner = Some(winner_id);
                ctx.notify(EndgameMessage {});
            }
            ClientCommand::Error(_) => (),
        }
    }
}

impl Handler<EndgameMessage> for Game {
    type Result = ();

    fn handle(&mut self, _: EndgameMessage, ctx: &mut Self::Context) -> Self::Result {
        self.game_state.state = State::Ended;
        let iter = self.addrs.iter();
        let winner = self.game_state.winner;
        let mut msg = "Draw";
        for (user_id, addr) in iter {
            if let Some(winner_id) = winner {
                if *user_id == winner_id {
                    msg = "Victory!";
                } else {
                    msg = "Defeat";
                }
            }
            addr.do_send(ServerResponseMessage::new("result", msg));
        }
        let iter = self.timers.iter();
        for (_, addr) in iter {
            addr.do_send(StopTimer {});
        }
        let game = CompletedGame {
            game_id: self.id,
            winner: self.game_state.winner,
            player1_id: self.game_state.x_data.user_id,
            player2_id: self.game_state.o_data.user_id,
            player1_elo: self.game_state.x_data.elo,
            player2_elo: self.game_state.o_data.elo,
            game_start_time: self.started_at,
            game_end_time: Utc::now(),
        };
        self.srv.do_send(GameEnded(game));
        ctx.stop();
    }
}
