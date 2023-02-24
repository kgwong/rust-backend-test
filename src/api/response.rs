use log::info;
use serde::{Deserialize, Serialize};
use serde::Serializer;

pub trait TMessageName{
    fn message_name() -> &'static str;
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TResponse {
    //pub message_name: MessageName,
    //pub response_type: ResponseType,
}

#[derive(Deserialize, Debug)]
pub enum GenericResponse<T: TMessageName> {
    Ok(T),
    ClientError(String),
    ServerError(String),
}

impl<T: TMessageName + Serialize> Serialize for GenericResponse<T>
 {

    fn serialize<S>(&self, serializer: S, ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("GenericResponse", 2)?;

        state.serialize_field("message_name", T::message_name())?;
        match self{
            GenericResponse::Ok(x) => {
                state.serialize_field("success", x)?;
            },
            GenericResponse::ClientError(v) => {
                state.serialize_field("client_error", v)?;
            },
            GenericResponse::ServerError(v) => {
                state.serialize_field("server_error", v)?;
            },
        }
        state.end()
    }
}