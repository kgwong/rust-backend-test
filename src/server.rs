use actix::prelude::*;
use std::{net, rc::Rc};

use log::info;

use crate::{game_manager, client_session::ClientSession};

use uuid::Uuid;

#[derive(Debug)]
pub struct ClientRequestWrapper<T: Message>{
    pub client_uuid: Uuid,
    pub peer_addr: net::SocketAddr,
    pub req: T,
    pub client_addr: Addr<ClientSession>
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

impl Handler<ClientRequestWrapper<crate::api::create_game::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<crate::api::create_game::Request>>;

    fn handle(
        &mut self,
        msg: ClientRequestWrapper<crate::api::create_game::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        let resp = self.gm.create_game(
            Rc::new(crate::player::PlayerClient{
                client_uuid: msg.client_uuid,
                peer_addr: msg.peer_addr,
                client_addr: msg.client_addr,
                name: msg.req.host_name
            }));
        match resp {
            Ok(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::Ok(
                        crate::api::create_game::Response{}
                    )
                ),
            Err(e) =>
                MessageResult(
                    crate::api::response::GenericResponse::ServerError("create_game error TODO".to_string())
                ),
        }
    }
}

impl Handler<ClientRequestWrapper<crate::api::join_game::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<crate::api::join_game::Request>>;

    fn handle(&
        mut self,
        msg: ClientRequestWrapper<crate::api::join_game::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        let player = Rc::new(crate::player::PlayerClient{
            client_uuid: msg.client_uuid,
            peer_addr: msg.peer_addr,
            client_addr: msg.client_addr,
            name: msg.req.player_name
        });
        match self.gm.join_game(player, &msg.req.room_code) {
            Ok(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::Ok(crate::api::join_game::Response{})),
            Err(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::ClientError("failed".to_string())),
        }
    }
}

impl Handler<ClientRequestWrapper<crate::api::start_game::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<crate::api::start_game::Request>>;

    fn handle(
        &mut self,
        msg: ClientRequestWrapper<crate::api::start_game::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        match self.gm.start_game(msg.client_uuid) {
            Ok(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::Ok(crate::api::start_game::Response{})),
            Err(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::ClientError("failed".to_string())),
        }
    }
}



impl Handler<ClientRequestWrapper<crate::api::set_player_ready::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<crate::api::set_player_ready::Request>>;

    fn handle(
        &mut self,
        msg: ClientRequestWrapper<crate::api::set_player_ready::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        match self.gm.set_player_ready(msg.client_uuid, msg.req.ready_state) {
            Ok(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::Ok(crate::api::set_player_ready::Response{})),
            Err(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::ClientError("failed".to_string())),
        }
    }
}


impl Handler<ClientRequestWrapper<crate::api::submit_drawing::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<crate::api::submit_drawing::Request>>;

    fn handle(
        &mut self,
        msg: ClientRequestWrapper<crate::api::submit_drawing::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        match self.gm.submit_drawing(msg.client_uuid, msg.req.drawing, msg.req.round) {
            Ok(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::Ok(crate::api::submit_drawing::Response{})),
            Err(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::ClientError("failed".to_string())),
        }
    }
}

impl Handler<ClientRequestWrapper<crate::api::vote::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<crate::api::vote::Request>>;

    fn handle(
        &mut self,
        msg: ClientRequestWrapper<crate::api::vote::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        match self.gm.vote(msg.client_uuid, msg.req.votes) {
            Ok(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::Ok(crate::api::vote::Response{})),
            Err(_) =>
                MessageResult(
                    crate::api::response::GenericResponse::ClientError("failed".to_string())),
        }
    }
}