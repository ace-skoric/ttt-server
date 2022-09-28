use std::{collections::HashMap, rc::Rc};

use serde::Serialize;
use serde_repr::Serialize_repr;

use ttt_db::PlayerData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr)]
#[repr(i16)]
pub enum Sign {
    X = 0,
    O = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr)]
#[repr(i16)]
pub enum State {
    Created = 0,
    Starting = 1,
    Running = 2,
    Ended = 3,
}

#[derive(Debug, Serialize, Clone)]
pub struct Player {
    #[serde(skip_serializing)]
    pub user_id: i64,
    pub username: String,
    pub elo: i64,
    pub sign: Sign,
}

impl Player {
    fn from_player_data(player: PlayerData, sign: Sign) -> Self {
        Self {
            user_id: player.user_id,
            username: player.username,
            elo: player.elo,
            sign,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GameState {
    pub winner: Option<i64>,
    pub board: [Option<Sign>; 9],
    pub state: State,
    pub turn_player: Option<i64>,
    pub x_data: Rc<Player>,
    pub o_data: Rc<Player>,
    #[serde(skip_serializing)]
    pub p_map: HashMap<i64, Rc<Player>>,
    #[serde(skip_serializing)]
    pub s_map: HashMap<Sign, Rc<Player>>,
}

impl GameState {
    pub(crate) fn new(players: (PlayerData, PlayerData)) -> Self {
        let board = [None; 9];
        let first_turn = rand::random::<bool>();
        let x_data = Player::from_player_data(
            match first_turn {
                false => players.0.clone(),
                true => players.1.clone(),
            },
            Sign::X,
        );
        let o_data = Player::from_player_data(
            match first_turn {
                true => players.0,
                false => players.1,
            },
            Sign::O,
        );
        let x_data = Rc::new(x_data);
        let o_data = Rc::new(o_data);
        let mut p_map = HashMap::new();
        p_map.insert(x_data.user_id, x_data.clone());
        p_map.insert(o_data.user_id, o_data.clone());
        let mut s_map = HashMap::new();
        s_map.insert(x_data.sign, x_data.clone());
        s_map.insert(o_data.sign, o_data.clone());
        let turn_player = None;
        let state = State::Created;
        Self {
            winner: None,
            board,
            state,
            turn_player,
            x_data,
            o_data,
            p_map,
            s_map,
        }
    }
    pub fn to_msg(&self, user_id: i64) -> UserGameState {
        UserGameState::from_state(user_id, self)
    }
    pub(crate) fn check_endgame(&mut self) -> bool {
        let triples: &[&[usize; 3]; 8] = &[
            &[0, 1, 2],
            &[3, 4, 5],
            &[6, 7, 8],
            &[0, 3, 6],
            &[1, 4, 7],
            &[2, 5, 8],
            &[0, 4, 8],
            &[2, 4, 6],
        ];
        for triple in triples {
            let vec = triple
                .iter()
                .map(|i| self.board[*i].clone())
                .collect::<Vec<Option<Sign>>>();
            if vec.iter().all(|sign| *sign == Some(Sign::X)) {
                self.winner = Some(self.x_data.user_id);
                return true;
            } else if vec.iter().all(|sign| *sign == Some(Sign::O)) {
                self.winner = Some(self.o_data.user_id);
                return true;
            }
        }
        if self.board.iter().all(|x| x.is_some()) {
            return true;
        }
        false
    }
    pub(crate) fn play(&mut self, user_id: i64, i: usize) -> bool {
        if self.board[i].is_some() {
            return false;
        }
        let sign = self.p_map.get(&user_id).unwrap().sign;
        self.board[i] = Some(sign);
        return true;
    }
}

#[derive(Debug, Clone, Copy, Serialize_repr)]
#[repr(i16)]
pub enum Winner {
    You = 0,
    Opponent = 1,
    Draw = 2,
    None = -1,
}

#[derive(Debug, Clone, Copy, Serialize_repr)]
#[repr(i16)]
pub enum UserPlayer {
    You = 0,
    Opponent = 1,
    None = -1,
}

#[derive(Debug, Serialize)]
pub struct UserGameState {
    pub winner: Winner,
    pub state: State,
    pub board: [Option<Sign>; 9],
    pub turn_player: UserPlayer,
    pub your_data: Rc<Player>,
    pub opp_data: Rc<Player>,
}

impl UserGameState {
    pub fn from_state(user_id: i64, state: &GameState) -> Self {
        let your_data = state.p_map.get(&user_id).unwrap().clone();
        let opp_data = match your_data.sign {
            Sign::X => state.o_data.clone(),
            Sign::O => state.x_data.clone(),
        };
        let turn_player = match state.turn_player {
            Some(id) => {
                if id == user_id {
                    UserPlayer::You
                } else {
                    UserPlayer::Opponent
                }
            }
            None => UserPlayer::None,
        };
        let board = state.board.clone();
        let winner = if state.winner.is_some() {
            let winner = state.winner.unwrap();
            if winner == user_id {
                Winner::You
            } else {
                Winner::Opponent
            }
        } else if board.into_iter().any(|x| x.is_none()) {
            Winner::None
        } else {
            Winner::Draw
        };
        let state = state.state;
        Self {
            winner,
            state,
            board,
            turn_player,
            your_data,
            opp_data,
        }
    }
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct Timers {
    pub you: f32,
    pub opp: f32,
}
