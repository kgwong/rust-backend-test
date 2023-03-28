use actix::prelude::*;
use serde::{Deserialize, Serialize};
use crate::game::{drawing::Drawing, errors::SubmitDrawingError};

use super::response::*;

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "ApiResponse<Response>")]
pub struct Request {
    // The player's contribution of the drawing
    pub drawing: Drawing,
    pub round: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response;

impl From<Result<(), SubmitDrawingError>> for ApiResponse<Response> {
    fn from(value: Result<(), SubmitDrawingError>) -> Self {
        match value {
            Ok(_) => {
                ApiResponse::Ok(Response)
            },
            Err(e) => {
                match e {
                    SubmitDrawingError::ClientIsNotInAGame =>
                        ApiResponse::ClientError("client is not in a game".to_string()),
                    SubmitDrawingError::DrawingSubmittedForWrongRound =>
                        ApiResponse::ClientError("drawing submitted for wrong round".to_string()),
                    SubmitDrawingError::DrawingWasAlreadySubmitted =>
                        ApiResponse::ClientError("drawing was already submitted for this round".to_string()),
                }
            }
        }
    }
}

impl MessageName for Response{
    fn message_name() -> &'static str {
        "submit_drawing"
    }
}