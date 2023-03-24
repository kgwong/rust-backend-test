use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{game_settings::GameSettings};

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct GameSettingsUpdate {
    pub message_name: String, //TODO
    #[serde(flatten)]
    pub settings: GameSettings,
}