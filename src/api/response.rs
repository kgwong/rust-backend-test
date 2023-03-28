use serde::{Deserialize, Serialize};
use serde::Serializer;

pub trait MessageName{
    fn message_name() -> &'static str;
}

#[derive(Deserialize, Debug)]
pub enum ApiResponse<T: MessageName> {
    Ok(T),
    ClientError(String),
    ServerError(String),
}

impl<T: MessageName + Serialize> Serialize for ApiResponse<T>
 {

    fn serialize<S>(&self, serializer: S, ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("GenericResponse", 2)?;

        state.serialize_field("message_name", T::message_name())?;
        match self{
            ApiResponse::Ok(x) => {
                state.serialize_field("success", x)?;
            },
            ApiResponse::ClientError(v) => {
                state.serialize_field("client_error", v)?;
            },
            ApiResponse::ServerError(v) => {
                state.serialize_field("server_error", v)?;
            },
        }
        state.end()
    }
}