use crossbeam_channel::Receiver;
use file_common::{file_bytes, ConnectionState, TestPacket};
use laminar::{Packet, Socket, SocketEvent};
use std::{collections::VecDeque, io, io::Write, net::SocketAddr, time::Instant};

pub struct Client {
    client_state: ConnectionState,
    _endpoint: SocketAddr,
    socket: Socket,
    receiver: Receiver<SocketEvent>,
    pub packet_chunks: VecDeque<(usize, Vec<u8>)>,
    pub total_length: usize,
    pub acked_packets: Vec<usize>,
    pub chunk_size: usize,
    pub server_addr: SocketAddr,
}

impl Client {
    pub fn new(to_sent: Vec<u8>, chunk_size: usize) -> Client {
        let client_addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
        let socket = Socket::bind(client_addr).unwrap();

        let packet_chunks = to_sent.chunks(chunk_size)
            .map(|x| x.to_vec())
            .enumerate()
            .map(|(i, e)| (i, e))
            .collect::<VecDeque<(usize, Vec<u8>)>>();

        Client {
            client_state: ConnectionState::Disconnected,
            _endpoint: client_addr,
            receiver: socket.get_event_receiver(),
            socket,
            acked_packets: Vec::new(),
            server_addr: "127.0.0.1:12346".parse().unwrap(),
            packet_chunks,
            chunk_size,
            total_length: to_sent.len()
        }
    }

    pub fn poll(&mut self) -> laminar::Result<()> {
        self.socket.manual_poll(Instant::now());


        self.receive_packet()?;

        if let ConnectionState::Disconnected = self.client_state {
            return self.connect();
        }

        self.sent_next_packet()
    }

    fn receive_packet(&mut self) -> laminar::Result<()> {
        if let Ok(event) = self.receiver.try_recv() {
            match event {
                SocketEvent::Packet(packet) => match TestPacket::deserialize(packet.payload()) {
                    TestPacket::Ack(ack_nmr) => {
                        match self.client_state {
                            ConnectionState::Disconnected => {
                                unreachable!("Can not be disconnected while disconnected");
                            }
                            ConnectionState::Connected => {
                                self.acked_packets.push(ack_nmr);
                            }
                        }
                        self.acked_packets.push(ack_nmr);
                    }
                    TestPacket::ConnectionAccepted => {
                        self.advance_state(ConnectionState::Connected);
                    }
                    _ => {}
                },
                SocketEvent::Connect(_) => {
                    self.advance_state(ConnectionState::Connected);
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn sent_next_packet(&mut self) -> laminar::Result<()> {
        if self.client_state != ConnectionState::Connected {
            unreachable!("Can not sent while disconnected");
        }

        if let Some((chunk_nmr, data)) = self.packet_chunks.pop_front() {
            self.socket.send(Packet::reliable_ordered(
                self.server_addr,
                TestPacket::Packet(chunk_nmr, data).serialize(),
                Some(0),
            ))?;
        }

        Ok(())
    }

    fn connect(&mut self) -> laminar::Result<()> {
        let packet = TestPacket::Connect(b"connect".to_vec());
        self.socket.send(Packet::reliable_unordered(
            self.server_addr,
            packet.serialize(),
        ))
    }

    fn advance_state(&mut self, state: ConnectionState) {
        self.client_state = state;
    }
}