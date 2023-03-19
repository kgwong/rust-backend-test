use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{game::{GameState}, player_view::PlayerView};

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct LobbyUpdate {
    pub message_name: String, //TODO
    pub room_code: String,
    pub state: GameState,
    pub round: Option<usize>,
    pub num_rounds: usize,
    pub players: std::vec::Vec<PlayerView>,
}