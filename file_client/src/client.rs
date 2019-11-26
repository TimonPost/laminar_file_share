use std::{collections::VecDeque, net::SocketAddr, time::Instant};

use crossbeam_channel::Receiver;
use file_common::{ConnectionState, TestPacket};
use laminar::{Packet, Socket, SocketEvent};

pub struct Client {
    client_state: ConnectionState,
    _endpoint: SocketAddr,
    socket: Socket,
    receiver: Receiver<SocketEvent>,
    packet_chunks: VecDeque<(usize, Vec<u8>)>,
    acked_packets: Vec<usize>,
    sent_packets: Vec<usize>,
    server_addr: SocketAddr,
}

impl Client {
    pub fn new(to_sent: Vec<u8>, chunk_size: usize) -> Client {
        let client_addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
        let socket = Socket::bind(client_addr).unwrap();

        let packet_chunks = to_sent
            .chunks(chunk_size)
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
            sent_packets: Vec::new(),
        }
    }

    /// Performs a client tick, this will receive acknowledgements, and sent the next chunk.
    pub fn tick(&mut self) -> laminar::Result<()> {
        self.socket.manual_poll(Instant::now());

        self.receive_packet()?;

        if let ConnectionState::Disconnected = self.client_state {
            return self.connect();
        }

        self.sent_next_chunk()
    }

    /// Receives and handles an packet from the server.
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

    /// Sends the next file chunk to server.
    fn sent_next_chunk(&mut self) -> laminar::Result<()> {
        if self.client_state != ConnectionState::Connected {
            unreachable!("Can not sent while disconnected");
        }

        if let Some((chunk_nmr, data)) = self.packet_chunks.pop_front() {
            self.socket.send(Packet::reliable_ordered(
                self.server_addr,
                TestPacket::Packet(chunk_nmr, data).serialize(),
                Some(1),
            ))?;

            self.sent_packets.push(chunk_nmr);
        }

        Ok(())
    }

    /// Sends a connection request to the server.
    fn connect(&mut self) -> laminar::Result<()> {
        let packet = TestPacket::Connect(b"connect".to_vec());
        self.socket.send(Packet::reliable_unordered(
            self.server_addr,
            packet.serialize(),
        ))
    }

    /// Advances state into a new state.
    fn advance_state(&mut self, state: ConnectionState) {
        self.client_state = state;
    }

    /// Returns the acknowledged packets.
    pub fn acked_packets(&self) -> &Vec<usize> {
        &self.acked_packets
    }

    /// Returns the sent packets.
    pub fn sent_packets(&self) -> &Vec<usize> {
        &self.sent_packets
    }

    /// Returns the number of chunks
    pub fn number_of_chunks(&self) -> usize {
        self.packet_chunks.len()
    }
}
