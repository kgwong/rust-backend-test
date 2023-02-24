use actix::prelude::*;
use log::info;

use crate::game_manager;


pub struct GameServer {
    gm: game_manager::GameManager
}


impl GameServer {
    pub fn new() -> Self {
        GameServer {
            gm: game_manager::GameManager::new(),
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<crate::api::create_game::Request> for GameServer {
    // type Result = std::result::Result<crate::api::create_game::Response, actix_web::Error>;
    type Result = MessageResult<crate::api::create_game::Request>;


    fn handle(&mut self, msg: crate::api::create_game::Request, _ctx: &mut Context<Self>) -> Self::Result {
        info!("create game received");

        let resp = self.gm.create_game(msg);
        MessageResult(resp)
    }
}
