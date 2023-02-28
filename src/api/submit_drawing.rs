use actix::prelude::*;
use serde::{Deserialize, Serialize};
use super::response::*;
use super::response::TMessageName;


type Stroke = std::vec::Vec<(i32, i32)>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
}

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "GenericResponse<Response>")]
pub struct Request {
    pub drawing: std::vec::Vec<Stroke>,
    pub round: usize,
}

impl TMessageName for Response{
    fn message_name() -> &'static str {
        "submit_drawing"
    }
}