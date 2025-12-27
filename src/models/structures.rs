use serde::{Deserialize, Serialize};
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct RecordedEvent {
    pub(crate) action_record: super::enums::ActionEnum,
    pub(crate) timestamp: i64,
}

pub struct IRRLClient {
    pub(crate) stream: TcpStream,
}
