use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::errors::PlayAgainError;

use super::response::*;

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "ApiResponse<Response>")]
pub struct Request{}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response;

impl From<Result<(), PlayAgainError>> for ApiResponse<Response> {
    fn from(value: Result<(), PlayAgainError>) -> Self {
        match value {
            Ok(_) => {
                ApiResponse::Ok(Response)
            },
            Err(e) => {
                match e {
                    PlayAgainError::ClientIsNotInAGame =>
                        ApiResponse::ClientError("client is not in a game".to_string()),
                    PlayAgainError::ClientIsNotTheHost =>
                        ApiResponse::ClientError("client is not the host".to_string()),
                    PlayAgainError::GameIsNotOver =>
                        ApiResponse::ClientError("game is not over".to_string()),
                }
            }
        }
    }
}

impl MessageName for Response{
    fn message_name() -> &'static str {
        "play_again"
    }
}