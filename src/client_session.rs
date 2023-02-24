use std::sync::{Mutex, Arc};
use actix::{Actor, StreamHandler, Addr, WrapFuture, ActorFutureExt, fut, ContextFutureSpawner};

use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use log::{info, error};

use serde_json::{Value};

use crate::server;

#[derive(Clone)]
pub struct ClientSession{
    pub server: Addr<server::GameServer>
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
                            "createGame"  =>  {
                                let req: crate::api::create_game::Request = serde_json::from_str(&text).expect("failed to parse");
                                let l = self.server
                                    .send(req)
                                    .into_actor(self)
                                    .then(|res, _, ctx|{
                                        let js_resp = serde_json::to_string(&res.unwrap()).expect("oops");
                                        ctx.text(js_resp);
                                        fut::ready(())
                                    });
                                l.wait(ctx);
                            }
                            "joinGame" => {
                                //let req = serde_json::from_str(&text).expect("failed to parse");
                                //let resp = shared_state.gm.join_game(req);
                                //let js_resp = serde_json::to_string(&resp).expect("oops");
                                //info!("{}", js_resp);
                                //ctx.text(js_resp);
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
                //TODO
            }
        }
    }
}
