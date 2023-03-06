use std::{net};

use actix::{Actor, StreamHandler, Addr, WrapFuture, ActorFutureExt, fut, ContextFutureSpawner, Message, AsyncContext, Handler, Context};
use actix_web_actors::ws;

use log::{info, error};

use serde_json::{Value};

use crate::server::{self, ClientRequestWrapper};

use uuid::Uuid;

pub struct ClientSession{
    uuid: Uuid,
    server: Addr<server::GameServer>,
    peer_addr: net::SocketAddr,

}

impl ClientSession {
    pub fn new(server: Addr<server::GameServer>, peer_addr: net::SocketAddr) -> Self {
        ClientSession {
            uuid: Uuid::new_v4(),
            server,
            peer_addr,
        }
    }

    fn wrap_request<T: Message>(&self, req: T, ctx: &ws::WebsocketContext<Self>) -> server::ClientRequestWrapper<T> {
        ClientRequestWrapper{
            client_uuid: self.uuid,
            peer_addr: self.peer_addr,
            req: req,
            client_addr: ctx.address(),
        }
    }
}

impl Actor for ClientSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("New connection");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Connection closed");
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        info!("Message Received: {:?}", msg);
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
                            "vote" => {
                                let req: crate::api::vote::Request = serde_json::from_str(&text).expect("failed to parse");
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



impl Handler<crate::api::game_update::GameUpdate> for ClientSession {
    type Result = ();

    fn handle(
        &mut self,
        msg: crate::api::game_update::GameUpdate,
        ctx: &mut Self::Context)
    -> Self::Result {
        ctx.text(serde_json::to_string(&msg).expect("oops"));
    }
}
