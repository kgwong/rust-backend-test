use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use log::{info};

use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
struct Test{
    message_name: String
}

/// Define HTTP actor
struct MyWs;

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
                        info!("received valid message {}", message_name)
                    },
                    _ => info!("failure")
                }

                // let t = serde_json::from_str(&text);
                // t.



                // info!("{:?}", v);


                let t: Test = match serde_json::from_str(&text)  {
                    Ok(test) => test,
                    Err(error) => panic!("ERROR: {}", error)
                };

                info!("{:?}", t);

                ctx.text(text)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
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