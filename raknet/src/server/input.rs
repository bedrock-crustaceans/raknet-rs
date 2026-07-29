use crate::session::RakSessionId;
use std::net::SocketAddr;
use std::time::SystemTime;

pub enum RakServerInput {
    SetMessage(Box<[u8]>),
    SetMaxConnections(usize),
    Datagram(Box<[u8]>, SocketAddr, SystemTime),
    RemoveSession(RakSessionId),
}
