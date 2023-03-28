use std::collections::HashMap;

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game::errors::SubmitVoteError;

use super::response::*;

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "ApiResponse<Response>")]
pub struct Request {
    pub votes: HashMap<Uuid, i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response;

impl From<Result<(), SubmitVoteError>> for ApiResponse<Response> {
    fn from(value: Result<(), SubmitVoteError>) -> Self {
        match value {
            Ok(_) => {
                ApiResponse::Ok(Response)
            },
            Err(e) => {
                match e {
                    SubmitVoteError::ClientIsNotInAGame =>
                        ApiResponse::ClientError("client is not in a game".to_string()),
                    SubmitVoteError::GameHasNotStarted =>
                        ApiResponse::ClientError("game has not started".to_string()),
                    SubmitVoteError::MaximumVotesExceeded =>
                        ApiResponse::ClientError("maximum votes exceeded".to_string()),
                    SubmitVoteError::ClientVotedForSelf =>
                        ApiResponse::ClientError("client cannot vote for their own drawing".to_string()),
                    SubmitVoteError::InvalidDrawingId =>
                        ApiResponse::ClientError("votes included an invalid drawing id".to_string())
                }
            }
        }
    }
}

impl MessageName for Response{
    fn message_name() -> &'static str {
        "submit_vote"
    }
}