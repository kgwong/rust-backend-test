use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game::drawing::Drawing;

// TODO: probably don't implement clone
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BallotItem {
    pub id: Uuid,
    pub suggestion: String,
    pub drawing: Drawing,
}

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct VotingBallot {
    // TODO: add is_votable so client knows its their own
    pub message_name: String, //TODO
    pub round: usize,
    pub ballot: Vec<BallotItem>,
}