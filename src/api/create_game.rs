use actix::prelude::*;
use serde::{Deserialize, Serialize};
use super::response::*;
use super::response::TMessageName;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    //TODO replace with game state
    pub room_code: String
}


#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "GenericResponse<Response>")]
pub struct Request {
    pub host_name: String,
}

impl TMessageName for Response{
    fn message_name() -> &'static str {
        "create_game"
    }
}