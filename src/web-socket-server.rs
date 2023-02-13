use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use log::{info};

use serde_json::{Value};

mod api;
mod game_manager;
mod room_code_generator;
mod game;

/// Define HTTP actor
struct MyWs{
    gm: game_manager::GameManager,
}

impl MyWs{

    pub fn new() -> Self {
        MyWs { 
            gm: game_manager::GameManager::new(),
        }
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
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
                        match message_name.as_str() {

                            "createGame" => {
                                let req = serde_json::from_str(&text).expect("failed to parse");
                                let resp = self.gm.create_game(req);
                                let js_resp = serde_json::to_string(&resp).expect("oops");
                                ctx.text(js_resp);
                            }
                            "joinGame" => {
                                let req = serde_json::from_str(&text).expect("failed to parse");
                                let resp = self.gm.join_game(req);
                                let js_resp = serde_json::to_string(&resp).expect("oops");
                                ctx.text(js_resp);
                            }

                            _ => info!("Unknown message {}", message_name)
                        }

                    },
                    _ => info!("failure")
                }
            }
            _ => {
                // TODO 
            },
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs::new(), &req, stream);
    info!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}