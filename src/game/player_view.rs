use std::rc::Rc;

use serde::{Serialize, Deserialize};

use crate::websocket::player::PlayerClient;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PlayerState {
    NotReady,
    Ready,
    Drawing,
    DrawingDone,
    Voting,
    VotingDone,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerView {
    pub name: String,
    pub state: PlayerState,
    pub score: i32,
}

#[derive(Debug)]
pub struct Player{
    pub client: Rc<PlayerClient>,
    pub state: PlayerState,
    pub score: i32,
}

impl Player{
    pub fn new(client: Rc<PlayerClient>) -> Player {
        Player {
            client: client,
            state: PlayerState::NotReady,
            score: 0,
        }
    }

    pub fn to_view(&self) -> PlayerView {
        PlayerView {
            name: self.client.name.clone(),
            state: self.state,
            score: self.score,
        }
    }
}
