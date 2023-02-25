use std::net;

use uuid::Uuid;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player{
    pub client_uuid: Uuid,
    pub peer_addr: net::SocketAddr,
    pub name: String,
}