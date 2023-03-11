use std::rc::Rc;

use serde::{Serialize, Deserialize};

use crate::websocket::player::PlayerClient;

// TODO, make this an actual view, and not just copy fields?
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerView {
    pub name: String,
    pub ready_state: bool,
    pub score: i32,
}

#[derive(Debug)]
pub struct Player{
    pub client: Rc<PlayerClient>,
    pub ready_state: bool,
    pub score: i32,
}

impl Player{
    pub fn new(client: Rc<PlayerClient>) -> Player {
        Player {
            client: client,
            ready_state: false,
            score: 0,
        }
    }

    pub fn to_view(&self) -> PlayerView {
        PlayerView {
            name: self.client.name.clone(),
            ready_state: self.ready_state,
            score: self.score,
        }
    }
}
