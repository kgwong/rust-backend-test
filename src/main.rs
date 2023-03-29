use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use log::{info};

use crate::websocket::*;

mod api;
mod game;
mod websocket;

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<server::GameServer>>
) -> Result<HttpResponse, Error> {
    info!("Connection from: {}", req.peer_addr().expect("oops missing addr?"));
    //info!("Headers: {:?}", req.headers());

    let session = client_session::ClientSession::new(
        server.get_ref().clone(),
        req.peer_addr().expect("oops")
    );
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
        // TODO: bind via env-var
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}