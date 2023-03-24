use std::{net};

use actix::*;
use actix_web_actors::ws;

use log::{info};

use serde_json::{Value};

use crate::api::server_messages::*;
use crate::game::game_settings;
use crate::server::{self, ClientRequestWrapper};
use crate::websocket::server::ClientDisconnectMessage;

use uuid::Uuid;

use super::client_connection::ClientConnection;

pub struct ClientSession{
    id: Uuid,
    server: Addr<server::GameServer>,
    peer_addr: net::SocketAddr,

}

impl ClientSession {
    pub fn new(server: Addr<server::GameServer>, peer_addr: net::SocketAddr) -> Self {
        ClientSession {
            id: Uuid::new_v4(),
            server,
            peer_addr,
        }
    }

    fn wrap_request<T: Message>(&self, req: T, ctx: &ws::WebsocketContext<Self>) -> server::ClientRequestWrapper<T> {
        ClientRequestWrapper{
            client_connection: ClientConnection {
                id: self.id,
                peer_addr: self.peer_addr,
                actor_addr: ctx.address(),
            },
            req: req,
        }
    }
}

impl Actor for ClientSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("New connection {} from {}", self.id, self.peer_addr);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.server
            .do_send(ClientDisconnectMessage{
                client_id: self.id
            });
        info!("Connection closed {}", self.id);
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        info!("Message Received from {}: {:?}", self.id, msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let json: Value = match serde_json::from_str(&text) {
                    Ok(x) => x,
                    Err(error) => panic!("ERROR: {}", error)
                };
                match &json["message_name"] {
                    Value::String(message_name) => {
                        // let mut shared_state = self.shared_state.lock().unwrap();
                        match message_name.as_str() {
                            "create_game"  =>  {
                                let req: crate::api::create_game::Request = serde_json::from_str(&text).expect("failed to parse");
                                let l = self.server
                                    .send(self.wrap_request(req, ctx))
                                    .into_actor(self)
                                    .then(|res, _, ctx|{
                                        let js_resp = serde_json::to_string(&res.unwrap()).expect("oops");
                                        ctx.text(js_resp);
                                        fut::ready(())
                                    });
                                l.wait(ctx);
                            }
                            "join_game" => {
                                let req: crate::api::join_game::Request = serde_json::from_str(&text).expect("failed to parse");
                                let l = self.server
                                    .send(self.wrap_request(req, ctx))
                                    .into_actor(self)
                                    .then(|res, _, ctx|{
                                        let js_resp = serde_json::to_string(&res.unwrap()).expect("oops");
                                        ctx.text(js_resp);
                                        fut::ready(())
                                    });
                                l.wait(ctx);
                            }
                            "start_game" => {
                                let req: crate::api::start_game::Request = serde_json::from_str(&text).expect("failed to parse");
                                let l = self.server
                                    .send(self.wrap_request(req, ctx))
                                    .into_actor(self)
                                    .then(|res, _, ctx|{
                                        let js_resp = serde_json::to_string(&res.unwrap()).expect("oops");
                                        ctx.text(js_resp);
                                        fut::ready(())
                                    });
                                l.wait(ctx);
                            }
                            "set_player_ready" => {
                                let req: crate::api::set_player_ready::Request = serde_json::from_str(&text).expect("failed to parse");
                                let l = self.server
                                    .send(self.wrap_request(req, ctx))
                                    .into_actor(self)
                                    .then(|res, _, ctx|{
                                        let js_resp = serde_json::to_string(&res.unwrap()).expect("oops");
                                        ctx.text(js_resp);
                                        fut::ready(())
                                    });
                                l.wait(ctx);
                            }
                            "submit_drawing" => {
                                let req: crate::api::submit_drawing::Request = serde_json::from_str(&text).expect("failed to parse");
                                let l = self.server
                                    .send(self.wrap_request(req, ctx))
                                    .into_actor(self)
                                    .then(|res, _, ctx|{
                                        let js_resp = serde_json::to_string(&res.unwrap()).expect("oops");
                                        ctx.text(js_resp);
                                        fut::ready(())
                                    });
                                l.wait(ctx);
                            }
                            "submit_vote" => {
                                let req: crate::api::submit_vote::Request = serde_json::from_str(&text).expect("failed to parse");
                                let l = self.server
                                    .send(self.wrap_request(req, ctx))
                                    .into_actor(self)
                                    .then(|res, _, ctx|{
                                        let js_resp = serde_json::to_string(&res.unwrap()).expect("oops");
                                        ctx.text(js_resp);
                                        fut::ready(())
                                    });
                                l.wait(ctx);
                            }
                            _ => info!("Unknown message {}", message_name)
                        }
                    },
                    _ => info!("failure")
                }
            }
            Err(e) => {
                eprintln!("Failed to handle message: {}", e);
                ctx.text("Internal Server Error");
            },
            _ => {
                //TODO\
            }
        }
    }
}



impl Handler<lobby_update::LobbyUpdate> for ClientSession {
    type Result = ();

    fn handle(
        &mut self,
        msg: lobby_update::LobbyUpdate,
        ctx: &mut Self::Context)
    -> Self::Result {
        ctx.text(serde_json::to_string(&msg).expect("oops"));
    }
}

impl Handler<drawing_parameters::DrawingParameters> for ClientSession {
    type Result = ();

    fn handle(
        &mut self,
        msg: drawing_parameters::DrawingParameters,
        ctx: &mut Self::Context)
    -> Self::Result {
        ctx.text(serde_json::to_string(&msg).expect("oops"));
    }
}

impl Handler<voting_ballot::VotingBallot> for ClientSession {
    type Result = ();

    fn handle(
        &mut self,
        msg: voting_ballot::VotingBallot,
        ctx: &mut Self::Context)
    -> Self::Result {
        ctx.text(serde_json::to_string(&msg).expect("oops"));
    }
}

impl Handler<game_settings_update::GameSettingsUpdate> for ClientSession {
    type Result = ();

    fn handle(
        &mut self,
        msg: game_settings_update::GameSettingsUpdate,
        ctx: &mut Self::Context)
    -> Self::Result {
        ctx.text(serde_json::to_string(&msg).expect("oops"));
    }
}