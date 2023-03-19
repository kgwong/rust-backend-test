use std::collections::HashMap;
use std::rc::Rc;

use log::{trace, warn};
use uuid::Uuid;

use crate::game::{drawing::Drawing,game::{Game, JoinGameError}, room_code_generator::RoomCodeGenerator};

#[derive(Debug)]
pub struct CreateGameError;

#[derive(Debug)]
pub struct StartGameError;

#[derive(Debug)]
pub struct ReadyPlayerError;

#[derive(Debug)]
pub struct SubmitDrawingError;

#[derive(Debug)]
pub struct SubmitVoteError;

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
    -> Result<String, CreateGameError> {
        if self.is_already_in_a_game(&client_connection.id) {
            return Err(CreateGameError{});
        }
        let room_code = self.room_code_generator.generate();

        let game = Game::new(room_code.clone(), client_connection.clone(), name);
        game.broadcast_update();
        self.games_by_room_code.insert(room_code.clone(), game);
        self.room_code_by_client_id.insert(client_connection.id, room_code.clone());
        trace!("Games: {:?}", self.games_by_room_code);
        Ok(room_code)
    }

    pub fn join_game(
        &mut self,
        client_connection: Rc<crate::client_connection::ClientConnection>,
        room_code: &str,
        proposed_name: &str,
    ) -> Result<(), JoinGameError> {
        if self.is_already_in_a_game(&client_connection.id) {
            return Err(JoinGameError{});
        }

        trace!("Games: {:?}", self.games_by_room_code);
        let game = self.games_by_room_code.get_mut(room_code).ok_or_else(|| JoinGameError)?;
        self.room_code_by_client_id.insert(client_connection.id, room_code.to_string());
        game.add_player(client_connection, proposed_name)
    }

    pub fn set_player_ready(&mut self, client_id: &Uuid, ready_state: bool)
    -> Result<(), ReadyPlayerError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| ReadyPlayerError)?;
        game.set_player_ready(client_id, ready_state).map_err(|_| ReadyPlayerError)

    }

    pub fn start_game(&mut self, client_id: &Uuid) -> Result<(), StartGameError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| StartGameError)?;
        game.start_game(*client_id).map_err(|_| StartGameError)
    }

    pub fn submit_drawing(&mut self, client_id: &Uuid, drawing: Drawing, round: usize)
    -> Result<(), SubmitDrawingError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| SubmitDrawingError)?;
        game.submit_drawing(client_id, drawing, round).map_err(|_| SubmitDrawingError)
    }

    pub fn vote(&mut self, client_id: &Uuid, votes: HashMap<Uuid, i32>)
    -> Result<(), SubmitVoteError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| SubmitVoteError)?;
        game.submit_vote(client_id, votes).map_err(|_| SubmitVoteError)
    }

    pub fn remove_player_connection(&mut self, client_id: &Uuid) {
        if let Some(game) = self.get_game_mut(client_id) {
            game.disconnect_player(client_id);
        } else {
            warn!("Player was not connnected to a game: {}", client_id)
        }

        self.room_code_by_client_id.remove(client_id);
    }

    fn is_already_in_a_game(&self, client_id: &Uuid) -> bool {
        self.room_code_by_client_id.contains_key(client_id)
    }
}