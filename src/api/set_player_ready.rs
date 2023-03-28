use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::errors::SetPlayerReadyError;

use super::response::*;

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "ApiResponse<Response>")]
pub struct Request {
    pub ready_state: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response;

impl From<Result<(), SetPlayerReadyError>> for ApiResponse<Response> {
    fn from(value: Result<(), SetPlayerReadyError>) -> Self {
        match value {
            Ok(_) => {
                ApiResponse::Ok(Response)
            },
            Err(e) => {
                match e {
                    SetPlayerReadyError::ClientIsNotInAGame =>
                        ApiResponse::ClientError("client is not in a game".to_string()),
                }
            }
        }
    }
}

impl MessageName for Response{
    fn message_name() -> &'static str {
        "set_player_ready"
    }
}