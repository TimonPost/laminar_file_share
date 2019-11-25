use crossbeam_channel::Receiver;
use file_common::{file_bytes, TestPacket};
use laminar::{Packet, Socket, SocketEvent};
use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    net::SocketAddr,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct ClientEntry {
    pub received_bytes: Vec<u8>,
    pub endpoint: SocketAddr,
    pub total_bytes: usize,
}

impl ClientEntry {
    pub fn new(endpoint: SocketAddr, total_bytes: usize) -> ClientEntry {
        ClientEntry {
            received_bytes: Vec::new(),
            endpoint,
            total_bytes,
        }
    }
}

pub struct Server {
    _endpoint: SocketAddr,
    socket: Socket,
    receiver: Receiver<SocketEvent>,
    pub(crate) clients: HashMap<SocketAddr, ClientEntry>,
    pub(crate) file_len: usize,
}

impl Server {
    pub fn new() -> Server {
        let socket = Socket::bind("127.0.0.1:12346").unwrap();

        let file_len = file_bytes().unwrap().len();

        Server {
            _endpoint: "127.0.0.1:12346".parse().unwrap(),
            receiver: socket.get_event_receiver(),
            socket,
            clients: HashMap::new(),
            file_len,
        }
    }

    pub fn poll(&mut self) -> laminar::Result<()> {
        self.socket.manual_poll(Instant::now());

        match self.receiver.try_recv() {
            Ok(event) => match event {
                SocketEvent::Packet(packet) => match TestPacket::deserialize(packet.payload()) {
                    TestPacket::Connect(data) => {
                        self.socket.send(Packet::reliable_ordered(
                            packet.addr(),
                            TestPacket::ConnectionAccepted.serialize(),
                            None,
                        ))?;
                    }
                    TestPacket::Packet(nmr, data) => {
                        let file_len = self.file_len;

                        let client = self
                            .clients
                            .entry(packet.addr())
                            .or_insert_with(|| ClientEntry::new(packet.addr(), file_len));

                        client.received_bytes.extend_from_slice(&data);

                        self.socket.send(Packet::reliable_ordered(
                            client.endpoint,
                            TestPacket::Ack(nmr).serialize(),
                            None,
                        ))?;

                        if client.received_bytes.len() >= self.file_len {
                            let mut file = File::create(format!(
                                "./result_{}_{}.jpg",
                                client.endpoint.ip(),
                                client.endpoint.port()
                            ))?;
                            file.write(&client.received_bytes)?;
                        }
                    }
                    TestPacket::Disconnect => {}
                    _ => {}
                },
                SocketEvent::Connect(addr) => {}
                SocketEvent::Timeout(addr) => {}
            },
            Err(e) => {}
        }

        Ok(())
    }
}
