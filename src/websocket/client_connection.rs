use std::net;

use actix::Addr;
use uuid::Uuid;

use crate::client_session::ClientSession;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientConnection{
    pub id: Uuid,
    // Address to the client host
    pub peer_addr: net::SocketAddr,
    // Actix address to the ClientSession actor
    pub actor_addr: Addr<ClientSession>,
}