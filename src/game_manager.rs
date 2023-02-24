use log::info;

use crate::api::{*, self};
use crate::game::Game;
use crate::room_code_generator::RoomCodeGenerator;

#[derive(Clone)]
pub struct GameManager {
    room_code_generator: RoomCodeGenerator,
    games: std::collections::HashMap<String, Game>,
}

impl GameManager {

    pub fn new() -> Self {
        GameManager {
            room_code_generator: RoomCodeGenerator::new(4),
            games: std::collections::HashMap::new(),
        }
    }

    pub fn create_game(&mut self, req: create_game::Request) -> create_game::Response {
        let room_code = self.room_code_generator.generate();

        //figure out the move stuff
        let rc2 = room_code.clone();
        self.games.insert(room_code, Game::new(req.host_name));
        info!("Games: {:?}", self.games);
        create_game::Response{ message_name: create_game::MessageName::Foo, status_code: 200, room_code: rc2}
    }

    pub fn join_game(&mut self, req: join_game::Request) -> api::response::GenericResponse<join_game::Response> {
        info!("Games: {:?}", self.games);
        let game = match self.games.get_mut(&req.room_code) {
            Some(g) => g,
            None => {
                info!("Room code does not exist: {}", &req.room_code);
                return api::response::GenericResponse::ClientError("Room does not exist".to_string());
            },
        };

        match game.add_player(req.player_name) {
            Ok(_) =>
                api::response::GenericResponse::Ok(
                    api::join_game::Response{
                        test: 123,
                    }
                ),
            Err(_) => {
                info!("Room is full");
                api::response::GenericResponse::ClientError("Room is full".to_string())
            }
        }
    }

    pub fn ready_player() {

    }

    pub fn start_game(&mut self, req: start_game::Request) -> api::response::GenericResponse<start_game::Response> {
        api::response::GenericResponse::ServerError("Not Implemented".to_string())
    }

    pub fn submit_drawing() {

    }

    pub fn vote() {

    }


}