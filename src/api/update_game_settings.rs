use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{game_settings::GameSettings, errors::UpdateGameSettingsError};

use super::response::*;


#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "ApiResponse<Response>")]
pub struct Request {
    pub game_settings: GameSettings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response;

impl From<Result<(), UpdateGameSettingsError>> for ApiResponse<Response> {
    fn from(value: Result<(), UpdateGameSettingsError>) -> Self {
        match value {
            Ok(_) => {
                ApiResponse::Ok(Response)
            },
            Err(e) => {
                match e {
                    UpdateGameSettingsError::ClientIsNotInAGame =>
                        ApiResponse::ClientError("client is not in a game".to_string()),
                    UpdateGameSettingsError::ClientIsNotTheHost =>
                        ApiResponse::ClientError("client is not the host".to_string()),
                    UpdateGameSettingsError::GameAlreadyStarted =>
                        ApiResponse::ClientError("game already started".to_string()),
                    UpdateGameSettingsError::InvalidNumRounds =>
                        ApiResponse::ClientError("rounds invalid".to_string()),
                    UpdateGameSettingsError::InvalidDrawingTimeLimit =>
                        ApiResponse::ClientError("drawing_phase_time_limit invalid".to_string()),
                    UpdateGameSettingsError::InvalidVotingTimeLimit =>
                        ApiResponse::ClientError("voting_phase_time_limit invalid".to_string()),
                    UpdateGameSettingsError::DeckDoesNotExist =>
                        ApiResponse::ClientError("deck does not exist".to_string()),
                    UpdateGameSettingsError::SettingRemovesAllDecks =>
                        ApiResponse::ClientError("cannot have 0 decks".to_string()),
                }
            }
        }
    }
}

impl MessageName for Response{
    fn message_name() -> &'static str {
        "update_game_settings"
    }
}