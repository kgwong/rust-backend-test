use actix::prelude::*;
use serde::{Deserialize, Serialize};

use super::response::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
}


#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "GenericResponse<Response>")]
pub struct Request {
    pub host_player_name: String,
}

impl TMessageName for Response{
    fn message_name() -> &'static str {
        "create_game"
    }
}