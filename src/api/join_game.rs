use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::errors::JoinGameError;

use super::response::*;

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "ApiResponse<Response>")]
pub struct Request {
    pub room_code: String,
    pub player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response;

impl From<Result<(), JoinGameError>> for ApiResponse<Response> {
    fn from(value: Result<(), JoinGameError>) -> Self {
        match value {
            Ok(_) => {
                ApiResponse::Ok(Response)
            },
            Err(e) => {
                match e {
                    JoinGameError::ClientIsAlreadyInAGame =>
                        ApiResponse::ClientError("client is already in a game".to_string()),
                    JoinGameError::RoomDoesNotExist =>
                        ApiResponse::ClientError("room does not exist".to_string()),
                    JoinGameError::GameFull =>
                        ApiResponse::ClientError("game is full".to_string()),
                    JoinGameError::GameAlreadyStarted =>
                        ApiResponse::ClientError("game already started".to_string()),
                }
            }
        }
    }
}

impl MessageName for Response{
    fn message_name() -> &'static str {
        "join_game"
    }
}