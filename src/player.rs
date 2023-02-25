use std::net;

use actix::Addr;
use uuid::Uuid;

use crate::client_session::ClientSession;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player{
    pub client_uuid: Uuid,
    pub peer_addr: net::SocketAddr,
    pub client_addr: Addr<ClientSession>,
    pub name: String,
}