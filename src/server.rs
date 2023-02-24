use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::{net};

use log::info;

use crate::game_manager;

use uuid::Uuid;

#[derive(Debug)]
pub struct ClientRequestWrapper<T: Message>{
    pub client_uuid: Uuid,
    pub peer_addr: net::SocketAddr,
    pub req: T,
}

impl<T: Message> Message for ClientRequestWrapper<T> {
    type Result = T::Result;
}

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
    type Result = MessageResult<crate::api::create_game::Request>;


    fn handle(&mut self, msg: crate::api::create_game::Request, _ctx: &mut Context<Self>) -> Self::Result {
        info!("create game received");

        let resp = self.gm.create_game(msg);
        MessageResult(resp)
    }
}

impl Handler<crate::api::join_game::Request> for GameServer {
    type Result = MessageResult<crate::api::join_game::Request>;

    fn handle(&mut self, msg: crate::api::join_game::Request, _ctx: &mut Context<Self>) -> Self::Result {
        info!("join game received");

        let resp = self.gm.join_game(msg);
        MessageResult(resp)
    }
}

impl Handler<ClientRequestWrapper<crate::api::start_game::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<crate::api::start_game::Request>>;

    fn handle(&mut self, msg: ClientRequestWrapper<crate::api::start_game::Request>, _ctx: &mut Context<Self>) -> Self::Result {
        info!("join game received");

        MessageResult(self.gm.start_game(msg.req))
    }
}
