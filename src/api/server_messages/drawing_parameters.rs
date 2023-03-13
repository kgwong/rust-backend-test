use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::drawing::Drawing;

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct DrawingParameters {
    pub message_name: String, //TODO
    pub round: usize,
    pub drawing_suggestion: String,
    pub imprint: Option<Drawing>,
}