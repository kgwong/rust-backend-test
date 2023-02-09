use serde::{Deserialize, Serialize};


// https://github.com/serde-rs/serde/issues/760#issuecomment-499570311
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageName {
    #[serde(rename = "createGame")] Foo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub host_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub message_name: MessageName,

    pub status_code: i32,

    //TODO replace with game state
    pub room_code: String
}
