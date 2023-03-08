use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct DrawingParameters {
    pub message_name: String, //TODO
    pub round: usize,
    pub drawing_suggestion: String,
}