use std::collections::HashMap;
use std::rc::Rc;

use log::{trace, warn, info};
use uuid::Uuid;

use crate::game::{drawing::Drawing,game::Game, room_code_generator::RoomCodeGenerator};

use super::{game_settings::GameSettings, errors::*};

pub struct GameManager {
    room_code_generator: RoomCodeGenerator,
    games_by_room_code: std::collections::HashMap<String, Game>,
    room_code_by_client_id: std::collections::HashMap<Uuid, String>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            room_code_generator: RoomCodeGenerator::new(4),
            games_by_room_code: std::collections::HashMap::new(),
            room_code_by_client_id: std::collections::HashMap::new(),
        }
    }

    pub fn get_game_mut(&mut self, client_id: &Uuid) -> Option<&mut Game>{
        let room_code = self.room_code_by_client_id.get(client_id)?;
        self.games_by_room_code.get_mut(room_code)
    }

    pub fn create_game(
        &mut self,
        client_connection: Rc<crate::client_connection::ClientConnection>,
        name: String,
    )
    -> Result<(), CreateGameError> {
        if self.is_already_in_a_game(&client_connection.id) {
            return Err(CreateGameError::ClientIsAlreadyInAGame);
        }
        let room_code = self.room_code_generator.generate();

        let game = Game::new(room_code.clone(), client_connection.clone(), name);
        self.games_by_room_code.insert(room_code.clone(), game);
        self.room_code_by_client_id.insert(client_connection.id, room_code.clone());
        info!("# of games: {}", self.games_by_room_code.len());
        Ok(())
    }

    pub fn join_game(
        &mut self,
        client_connection: Rc<crate::client_connection::ClientConnection>,
        room_code: &str,
        proposed_name: &str,
    ) -> Result<(), JoinGameError> {
        if self.is_already_in_a_game(&client_connection.id) {
            return Err(JoinGameError::ClientIsAlreadyInAGame);
        }

        trace!("Games: {:?}", self.games_by_room_code);
        let game = self.games_by_room_code.get_mut(room_code).ok_or(JoinGameError::RoomDoesNotExist)?;
        self.room_code_by_client_id.insert(client_connection.id, room_code.to_string());
        game.add_player(client_connection, proposed_name)
    }

    pub fn update_game_settings(&mut self, client_id: &Uuid, game_settings: &GameSettings)
    -> Result<(), UpdateGameSettingsError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| UpdateGameSettingsError::ClientIsNotInAGame)?;
        game.update_settings(client_id, &game_settings)
    }

    pub fn set_player_ready(&mut self, client_id: &Uuid, ready_state: bool)
    -> Result<(), SetPlayerReadyError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| SetPlayerReadyError::ClientIsNotInAGame)?;
        game.set_player_ready(client_id, ready_state);
        Ok(())
    }

    pub fn start_game(&mut self, client_id: &Uuid) -> Result<(), StartGameError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| StartGameError::ClientIsNotInAGame)?;
        game.start_game(client_id)
    }

    pub fn play_again(&mut self, client_id: &Uuid) -> Result<(), PlayAgainError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| PlayAgainError::ClientIsNotInAGame)?;
        game.play_again(client_id)
    }

    pub fn submit_drawing(&mut self, client_id: &Uuid, drawing: Drawing, round: usize)
    -> Result<(), SubmitDrawingError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| SubmitDrawingError::ClientIsNotInAGame)?;
        game.submit_drawing(client_id, drawing, round)
    }

    pub fn submit_vote(&mut self, client_id: &Uuid, votes: HashMap<Uuid, i32>)
    -> Result<(), SubmitVoteError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| SubmitVoteError::ClientIsNotInAGame)?;
        game.submit_vote(client_id, votes)
    }

    pub fn remove_player_connection(&mut self, client_id: &Uuid) {
        if let Some(room_code) = self.room_code_by_client_id.remove(client_id) {
            {
                let game = self.games_by_room_code.get_mut(&room_code).expect("game should exist");
                game.disconnect_player(client_id)
            }
            {
                let game = self.games_by_room_code.get(&room_code).expect("game should exist");
                if game.all_players_disconnected() {
                    self.games_by_room_code.remove(&room_code);
                    info!("# of games: {}", self.games_by_room_code.len())
                }
            }
        } else {
            warn!("Player was not connnected to a game: {}", client_id)
        }
    }

    fn is_already_in_a_game(&self, client_id: &Uuid) -> bool {
        self.room_code_by_client_id.contains_key(client_id)
    }
}