use crate::api::create_game::{Request, Response};
use crate::room_code_generator::generate_room_code;

struct GameState{
    room_code: String
}

struct Game{

}

pub struct GameManager {
    games: std::collections::HashMap<String, Game>
}

impl GameManager {

    pub fn new() -> Self {
        GameManager {
            games: std::collections::HashMap::new()
        }
    }

    pub fn create_game(&mut self, req: crate::api::create_game::Request) -> crate::api::create_game::Response {
        let room_code = generate_room_code();

        //figure out the move stuff
        let rc2 = room_code.clone();
        self.games.insert(room_code, Game{});
        Response{ status_code: 200, room_code: rc2}
    }

    pub fn join_game() {

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