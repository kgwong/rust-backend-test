use std::sync::{Mutex, Arc};

use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use log::{info, error};

use serde_json::{Value};

mod api;
mod game_manager;
mod room_code_generator;
mod game;

pub struct SharedState{
    gm: game_manager::GameManager
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            gm: game_manager::GameManager::new(),
        }
    }
}

#[derive(Clone)]
pub struct MyWs{
    shared_state: web::Data<Arc<Mutex<SharedState>>>
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("New connection");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Connection closed");
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
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
                        let mut shared_state = self.shared_state.lock().unwrap();
                        match message_name.as_str() {
                            "createGame" => {
                                let req = serde_json::from_str(&text).expect("failed to parse");
                                let resp = shared_state.gm.create_game(req);
                                let js_resp = serde_json::to_string(&resp).expect("oops");
                                ctx.text(js_resp);
                            }
                            "joinGame" => {
                                let req = serde_json::from_str(&text).expect("failed to parse");
                                let resp = shared_state.gm.join_game(req);
                                let js_resp = serde_json::to_string(&resp).expect("oops");
                                info!("{}", js_resp);
                                ctx.text(js_resp);
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


pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    shared_state: web::Data<Arc<Mutex<SharedState>>>
) -> Result<HttpResponse, Error> {
    info!("Connection from: {}", req.peer_addr().expect("oops missing addr?"));
    info!("Headers: {:?}", req.headers());

    let server = MyWs {
        shared_state: shared_state.clone()
    };
    let resp = ws::start(server, &req, stream);
    info!("index_resp: {:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    let shared_state = web::Data::new(Arc::new(Mutex::new(SharedState::new())));

    info!("init server");
    HttpServer::new(move ||
            App::new()
                .app_data(shared_state.clone())
                .route("/ws/", web::get().to(ws_route))
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}