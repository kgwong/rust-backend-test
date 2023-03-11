use std::collections::HashMap;

use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::response::*;
use super::response::TMessageName;


#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
}

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "GenericResponse<Response>")]
pub struct Request {
    pub votes: HashMap<Uuid, i32>,
}

impl TMessageName for Response{
    fn message_name() -> &'static str {
        "submit_vote"
    }
}