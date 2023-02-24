use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use log::{info, error};

use serde_json::{Value};

mod api;
mod game_manager;
mod room_code_generator;
mod game;
mod client_session;
mod server;


pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<server::GameServer>>
) -> Result<HttpResponse, Error> {
    info!("Connection from: {}", req.peer_addr().expect("oops missing addr?"));
    //info!("Headers: {:?}", req.headers());

    let session = client_session::ClientSession {
        server: server.get_ref().clone()
    };
    let resp = ws::start(session, &req, stream);
    info!("index_resp: {:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    let server = server::GameServer::new().start();

    info!("init server");
    HttpServer::new(move ||
            App::new()
                .app_data(web::Data::new(server.clone()))
                .route("/ws/", web::get().to(ws_route))
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}