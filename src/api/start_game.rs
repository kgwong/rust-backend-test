use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::errors::StartGameError;

use super::response::*;


#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "ApiResponse<Response>")]
pub struct Request{}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response;

impl From<Result<(), StartGameError>> for ApiResponse<Response> {
    fn from(value: Result<(), StartGameError>) -> Self {
        match value {
            Ok(_) => {
                ApiResponse::Ok(Response)
            },
            Err(e) => {
                match e {
                    StartGameError::ClientIsNotInAGame =>
                        ApiResponse::ClientError("client is not in a game".to_string()),
                    StartGameError::ClientIsNotTheHost =>
                        ApiResponse::ClientError("client is not the host".to_string()),
                    StartGameError::GameAlreadyStarted =>
                        ApiResponse::ClientError("game already started".to_string()),
                    StartGameError::MinimumPlayersNotReached =>
                        ApiResponse::ClientError("not enough players to start game".to_string()),
                }
            }
        }
    }
}

impl MessageName for Response{
    fn message_name() -> &'static str {
        "start_game"
    }
}