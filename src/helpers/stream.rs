use serde_json;
use std::io::Write;
use std::net::TcpStream;

use crate::config::stream::{HOST, PORT};
use crate::models::enums::ActionEnum;
use crate::models::structures::IRRLClient;

impl IRRLClient {
    pub fn connect() -> Self {
        let stream =
            TcpStream::connect(format!("{}:{}", HOST, PORT)).expect("Could not connect to server");

        stream.set_nodelay(true).expect("Failed to set TCP_NODELAY");

        Self { stream }
    }

    pub fn send_action(&mut self, action: ActionEnum) {
        let json = serde_json::to_string(&action).unwrap();

        self.stream.write_all(json.as_bytes()).unwrap();
        self.stream.write_all(b"\n").unwrap();
    }

    pub fn close(&mut self) {
        use std::net::Shutdown;
        let _ = self.stream.shutdown(Shutdown::Both);
    }
}
