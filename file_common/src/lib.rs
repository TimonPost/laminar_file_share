use serde::{Deserialize, Serialize};
use std::{fs::File, io, io::Read};

mod acknowledgement_board_view;

pub use acknowledgement_board_view::{AcknowledgementBoardView, Options};

#[derive(Serialize, Deserialize)]
pub enum TestPacket {
    Connect(Vec<u8>),
    Packet(usize, Vec<u8>),
    Disconnect,
    Ack(usize),
    ConnectionAccepted,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connected,
}

impl TestPacket {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn deserialize(buf: &[u8]) -> TestPacket {
        bincode::deserialize(buf).unwrap()
    }
}

pub fn file_bytes() -> io::Result<Vec<u8>> {
    let mut f = File::open("./src/foo.jpg")?;

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    Ok(buffer)
}
