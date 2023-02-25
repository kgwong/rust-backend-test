use std::rc::Rc;

use log::info;
use uuid::Uuid;

use crate::player::Player;



#[derive(Debug)]
pub struct JoinGameError;

#[derive(Debug)]
pub struct StartGameError;

const MAX_PLAYERS: usize = 8;

#[derive(Debug, Clone, PartialEq)]
enum GameState{
    WaitingForPlayers,
    DrawingPhase,
    VotingPlase,
    Results,
}

#[derive(Debug)]
pub struct Game{
    state: GameState,
    host_player: Rc<Player>,
    players: std::vec::Vec<Rc<Player>>,
}

impl Game {

    pub fn new(host_player: Rc<Player>) -> Self {
        Game {
            state: GameState::WaitingForPlayers,
            players: std::vec![host_player.clone()],
            host_player: host_player,
        }
    }

    pub fn add_player(&mut self, player: Rc<Player>) -> Result<(), JoinGameError> {
        if self.players.len() == MAX_PLAYERS {
            return Err(JoinGameError);
        }

        self.players.push(player);
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
            Ok(())
        } else {
            Err(StartGameError)
        }
    }

    pub fn broadcast_update(&self) {
        info!("Broadcasting update to all players");
        for x in &self.players {
            x.client_addr.do_send(
                crate::api::game_update::GameUpdate{
                    test: "test".to_string()
                });
        }
    }
}