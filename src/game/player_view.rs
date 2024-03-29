use std::rc::Rc;

use serde::{Serialize, Deserialize};

use crate::websocket::client_connection::ClientConnection;

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

    // this player is the host of the game
    pub is_host: bool,
    // the client receiving this corresponds to this player
    pub is_you: bool,
    // if this player's client connection closed mid-game
    pub is_disconnected: bool,
}

#[derive(Debug)]
pub struct Player{
    pub client: Rc<ClientConnection>,
    pub name: String,
    // The connected player with the lowest host_rank should be the host.
    // host_rank = 1 if first player, 2 if second, and so on
    pub host_rank: usize,
    pub state: PlayerState,
    pub score: i32,
    pub is_disconnected: bool,
}

impl Player{
    pub fn new(client: Rc<ClientConnection>, name: String, number: usize) -> Player {
        Player {
            client: client,
            name: name,
            host_rank: number,
            state: PlayerState::NotReady,
            score: 0,
            is_disconnected: false,
        }
    }

    pub fn to_view(&self, is_host: bool, is_you: bool) -> PlayerView {
        PlayerView {
            name: self.name.clone(),
            state: self.state,
            score: self.score,
            is_host,
            is_you,
            is_disconnected: self.is_disconnected,
        }
    }
}
