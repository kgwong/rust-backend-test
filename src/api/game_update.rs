use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::GameState;

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct GameUpdate {
    pub room_code: String,
    pub state: GameState,
    pub round: usize,
    pub players: std::vec::Vec<String>,
}