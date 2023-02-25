use std::rc::Rc;

use log::info;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{player::Player, api::game_update::GameUpdate};



#[derive(Debug)]
pub struct JoinGameError;

#[derive(Debug)]
pub struct StartGameError;

const MAX_PLAYERS: usize = 8;
const MAX_ROUNDS: usize = 5;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameState{
    WaitingForPlayers,
    DrawingPhase,
    VotingPlase,
    Results,
}

#[derive(Debug)]
pub struct Game{
    room_code: String,
    state: GameState,
    host_player: Rc<Player>,
    players: std::vec::Vec<Rc<Player>>,
    round: usize,
    max_rounds: usize,
}

impl Game {

    pub fn new(room_code: String, host_player: Rc<Player>) -> Self {
        Game {
            room_code: room_code,
            state: GameState::WaitingForPlayers,
            host_player: host_player.clone(),
            players: std::vec![host_player],
            round: 0,
            max_rounds: MAX_ROUNDS,
        }
    }

    pub fn add_player(&mut self, player: Rc<Player>) -> Result<(), JoinGameError> {
        if self.players.len() == MAX_PLAYERS {
            return Err(JoinGameError);
        }

        let resolved_player = Rc::new(Player{
            client_uuid: player.client_uuid,
            peer_addr: player.peer_addr,
            client_addr: player.client_addr.clone(),
            name: self.resolve_name(&player),
        });

        self.players.push(resolved_player);
        info!("CurrentPlayers: {:?}", self.players);
        self.broadcast_update();
        return Ok(());
    }

    pub fn start_game(&mut self, client_id: Uuid) -> Result<(), StartGameError> {
        if self.host_player.client_uuid == client_id {
            if self.state != GameState::WaitingForPlayers {
                return Err(StartGameError)
            }
            info!("Host is starting the game");
            self.state = GameState::DrawingPhase;
            self.broadcast_update();
            Ok(())
        } else {
            Err(StartGameError)
        }
    }

    pub fn broadcast_update(&self) {
        info!("Broadcasting update to all players");
        for p in &self.players {
            self.send_game_view_to_player(p);
        }
    }

    fn current_game_view(&self) -> GameUpdate {
        GameUpdate {
            room_code: self.room_code.clone(),
            state: self.state.clone(),
            round: self.round,
            players: self.players.iter().map(|p| p.name.clone()).collect()
        }
    }

    fn send_game_view_to_player(&self, player: &Player) {
        player.client_addr.do_send(self.current_game_view());
    }

    /**
     *  Returns a new name in the form of `name(1)` if it's a duplicate of an existing name
     */
    fn resolve_name(&self, player: &Player) -> String {
        let mut proposed_name = player.name.clone();
        let mut count = 1;
        while self.players.iter().any(|p| p.name == proposed_name) {
            proposed_name = format!("{}({})", player.name, count);
            count += 1;
        }
        proposed_name
    }
}