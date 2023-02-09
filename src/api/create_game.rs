use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub host_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub status_code: i32,

    //TODO replace with game state
    pub room_code: String
}
