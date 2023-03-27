use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::drawing::Drawing;

// probably don't implement clone
#[derive(Serialize, Deserialize, Debug, Message, Clone)]
#[rtype(result = "()")]
pub struct Results {
    pub message_name: String, //TODO
    pub highest_rated_drawing: Drawing,
    pub imprint: Option<Drawing>,
    pub num_votes: i32,
    pub drawing_suggestion: String,
}