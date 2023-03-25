use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::game_settings::GameSettings;

use super::response::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
}

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "GenericResponse<Response>")]
pub struct Request {
    pub game_settings: GameSettings,
}

impl TMessageName for Response{
    fn message_name() -> &'static str {
        "update_game_settings"
    }
}