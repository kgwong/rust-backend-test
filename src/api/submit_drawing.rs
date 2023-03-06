use actix::prelude::*;
use serde::{Deserialize, Serialize};
use crate::game::drawing::Drawing;

use super::response::*;
use super::response::TMessageName;


#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
}

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "GenericResponse<Response>")]
pub struct Request {
    pub drawing: Drawing,
    pub round: usize,
}

impl TMessageName for Response{
    fn message_name() -> &'static str {
        "submit_drawing"
    }
}