use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::errors::CreateGameError;

use super::response::*;

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "ApiResponse<Response>")]
pub struct Request {
    pub host_player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response;

impl From<Result<(), CreateGameError>> for ApiResponse<Response> {
    fn from(value: Result<(), CreateGameError>) -> Self {
        match value {
            Ok(_) => {
                ApiResponse::Ok(Response)
            },
            Err(e) => {
                match e {
                    CreateGameError::ClientIsAlreadyInAGame =>
                        ApiResponse::ClientError("client is already in a game".to_string()),
                }
            }
        }
    }
}

impl MessageName for Response{
    fn message_name() -> &'static str {
        "create_game"
    }
}