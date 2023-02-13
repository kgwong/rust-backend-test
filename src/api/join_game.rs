use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum MessageName {
    #[serde(rename = "joinGame")] Foo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OkResponse{
    pub test: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    Ok(OkResponse),
    ClientError,
    ServerError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub room_code: String,
    pub player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub message_name: MessageName,
    pub response_type: ResponseType,
}
