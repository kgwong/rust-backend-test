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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VotableBallotItem {
    #[serde(flatten)]
    pub ballot_item: BallotItem,
    pub is_voting_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct VotingBallot {
    pub message_name: String, //TODO
    pub round: usize,
    pub ballot: Vec<VotableBallotItem>,
}