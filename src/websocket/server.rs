use actix::prelude::*;
use log::info;
use std::{net, rc::Rc};

use crate::{api::*, game::game_manager, client_session::ClientSession};

use uuid::Uuid;

use super::client_connection::ClientConnection;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientDisconnectMessage{
    pub uuid: Uuid,
}

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
    gm: game_manager::GameManager,
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

impl Handler<ClientRequestWrapper<create_game::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<create_game::Request>>;

    fn handle(
        &mut self,
        msg: ClientRequestWrapper<create_game::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        let resp = self.gm.create_game(
            Rc::new(
                ClientConnection{
                    id: msg.client_uuid,
                    peer_addr: msg.peer_addr,
                    actor_addr: msg.client_addr,
                }),
            msg.req.host_player_name);
        match resp {
            Ok(_) =>
                MessageResult(
                    response::GenericResponse::Ok(
                        create_game::Response{}
                    )
                ),
            Err(_) =>
                MessageResult(
                    response::GenericResponse::ServerError("create_game error TODO".to_string())
                ),
        }
    }
}

impl Handler<ClientRequestWrapper<join_game::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<join_game::Request>>;

    fn handle(&
        mut self,
        msg: ClientRequestWrapper<join_game::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        let player_connection = Rc::new(
            ClientConnection{
                id: msg.client_uuid,
                peer_addr: msg.peer_addr,
                actor_addr: msg.client_addr,
        });
        match self.gm.join_game(player_connection, &msg.req.room_code, &msg.req.player_name) {
            Ok(_) =>
                MessageResult(
                    response::GenericResponse::Ok(join_game::Response{})),
            Err(_) =>
                MessageResult(
                    response::GenericResponse::ClientError("failed".to_string())),
        }
    }
}

impl Handler<ClientRequestWrapper<start_game::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<start_game::Request>>;

    fn handle(
        &mut self,
        msg: ClientRequestWrapper<start_game::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        match self.gm.start_game(&msg.client_uuid) {
            Ok(_) =>
                MessageResult(
                    response::GenericResponse::Ok(start_game::Response{})),
            Err(_) =>
                MessageResult(
                    response::GenericResponse::ClientError("failed".to_string())),
        }
    }
}

impl Handler<ClientRequestWrapper<set_player_ready::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<set_player_ready::Request>>;

    fn handle(
        &mut self,
        msg: ClientRequestWrapper<set_player_ready::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        match self.gm.set_player_ready(&msg.client_uuid, msg.req.ready_state) {
            Ok(_) =>
                MessageResult(
                    response::GenericResponse::Ok(set_player_ready::Response{})),
            Err(_) =>
                MessageResult(
                    response::GenericResponse::ClientError("failed".to_string())),
        }
    }
}


impl Handler<ClientRequestWrapper<submit_drawing::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<submit_drawing::Request>>;

    fn handle(
        &mut self,
        msg: ClientRequestWrapper<submit_drawing::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        match self.gm.submit_drawing(&msg.client_uuid, msg.req.drawing, msg.req.round) {
            Ok(_) =>
                MessageResult(
                    response::GenericResponse::Ok(submit_drawing::Response{})),
            Err(_) =>
                MessageResult(
                    response::GenericResponse::ClientError("failed".to_string())),
        }
    }
}

impl Handler<ClientRequestWrapper<submit_vote::Request>> for GameServer {
    type Result = MessageResult<ClientRequestWrapper<submit_vote::Request>>;

    fn handle(
        &mut self,
        msg: ClientRequestWrapper<submit_vote::Request>,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        match self.gm.vote(&msg.client_uuid, msg.req.votes) {
            Ok(_) =>
                MessageResult(
                    response::GenericResponse::Ok(submit_vote::Response{})),
            Err(_) =>
                MessageResult(
                    response::GenericResponse::ClientError("failed".to_string())),
        }
    }
}


impl Handler<ClientDisconnectMessage> for GameServer {
    type Result = MessageResult<ClientDisconnectMessage>;

    fn handle(
        &mut self,
        msg: ClientDisconnectMessage,
        _ctx: &mut Context<Self>)
    -> Self::Result {
        let uuid = msg.uuid;
        info!("Received client disconnect: {}", uuid);
        MessageResult(())
    }
}