use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct GameUpdate {
    pub test: String,
}