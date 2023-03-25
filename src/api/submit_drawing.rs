use actix::prelude::*;
use serde::{Deserialize, Serialize};
use crate::game::drawing::Drawing;

use super::response::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
}

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "GenericResponse<Response>")]
pub struct Request {
    // The player's contribution of the drawing
    pub drawing: Drawing,
    pub round: usize,
}

impl TMessageName for Response{
    fn message_name() -> &'static str {
        "submit_drawing"
    }
}