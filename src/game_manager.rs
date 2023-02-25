use std::rc::Rc;

use log::info;
use uuid::Uuid;

use crate::api::{*, self};
use crate::game::{Game, JoinGameError};
use crate::player::Player;
use crate::room_code_generator::RoomCodeGenerator;

#[derive(Debug)]
pub struct CreateGameError;

#[derive(Debug)]
pub struct StartGameError;

pub struct GameManager {
    room_code_generator: RoomCodeGenerator,
    games_by_room_code: std::collections::HashMap<String, Game>,
    games_by_client_id: std::collections::HashMap<Uuid, String>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            room_code_generator: RoomCodeGenerator::new(4),
            games_by_room_code: std::collections::HashMap::new(),
            games_by_client_id: std::collections::HashMap::new(),
        }
    }

    pub fn get_game_mut(&mut self, client_id: Uuid) -> Option<&mut Game>{
        let rc = self.games_by_client_id.get(&client_id)?;
        self.games_by_room_code.get_mut(rc)
    }

    pub fn create_game(&mut self, player: Rc<crate::player::Player>) -> Result<String, CreateGameError> {
        let room_code = self.room_code_generator.generate();

        //let game = Rc::new(Game::new(player.clone()));

        self.games_by_room_code.insert(room_code.clone(), Game::new(player.clone()));
        self.games_by_client_id.insert(player.client_uuid, room_code.clone());
        info!("Games: {:?}", self.games_by_room_code);
        Ok(room_code)
    }

    pub fn join_game(&mut self, player: Rc<crate::player::Player>, room_code: &str) -> Result<(), JoinGameError> {
        info!("Games: {:?}", self.games_by_room_code);
        let game = self.games_by_room_code.get_mut(room_code).ok_or_else(|| JoinGameError)?;
        game.add_player(player)
    }

    pub fn ready_player() {

    }

    pub fn start_game(&mut self, client_id: Uuid) -> Result<(), StartGameError> {
        let game = self.get_game_mut(client_id).ok_or_else(|| StartGameError)?;
        game.start_game(client_id).map_err(|_| StartGameError)
    }

    pub fn submit_drawing() {

    }

    pub fn vote() {

    }


}