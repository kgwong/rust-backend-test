use actix::prelude::*;
use serde::{Deserialize, Serialize};
use super::response::*;
use super::response::TMessageName;


#[derive(Serialize, Deserialize, Debug)]
pub enum MessageName {
    #[serde(rename = "joinGame")] Foo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    pub test: i64
}

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "GenericResponse<Response>")]
pub struct Request {
    pub room_code: String,
    pub player_name: String,
}

impl TMessageName for Response{
    fn message_name() -> &'static str {
        "joinGame"
    }
}