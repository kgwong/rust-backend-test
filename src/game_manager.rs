use crate::api::*;
use crate::game::Game;
use crate::room_code_generator::RoomCodeGenerator;

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
        create_game::Response{ message_name: create_game::MessageName::Foo, status_code: 200, room_code: rc2}
    }

    pub fn join_game(&mut self, req: join_game::Request) -> join_game::Response {
        let game = match self.games.get_mut(&req.room_code) {
            Some(g) => g,
            None =>
                return join_game::Response {
                    message_name: join_game::MessageName::Foo,
                    response_type: join_game::ResponseType::ClientError},
        };

        match game.add_player(req.player_name) {
            Ok(_) =>
                join_game::Response{
                message_name: join_game::MessageName::Foo,
                response_type: join_game::ResponseType::Ok(
                    join_game::OkResponse{ test: 123} )},
            Err(_) =>
                join_game::Response{
                    message_name: join_game::MessageName::Foo,
                    response_type: join_game::ResponseType::ClientError },
        }
    }

    pub fn ready_player() {

    }

    pub fn start_game() {

    }

    pub fn submit_drawing() {

    }

    pub fn vote() {

    }


}