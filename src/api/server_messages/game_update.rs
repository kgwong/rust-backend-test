use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{game::{GameState}, player_view::PlayerView};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientInfo {
    pub player_index: usize,
}

// rename to lobby update?
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct GameUpdate {
    pub message_name: String, //TODO
    pub room_code: String,
    pub state: GameState,
    pub round: usize,
    pub players: std::vec::Vec<PlayerView>,

    pub client_info: ClientInfo,
}