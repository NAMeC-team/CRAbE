use std::io::Cursor;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc;
use crate::constants::BUFFER_SIZE;

pub struct MulticastClient<T: prost::Message> {
    socket: UdpSocket,
    buffer: [u8; BUFFER_SIZE],
    tx: mpsc::Sender<T>,
}

impl<T: prost::Message + Default> MulticastClient<T> {
    pub fn with_cli(tx: mpsc::Sender<T>, ip: String, port: u32) -> Self {
        let ipv4 = Ipv4Addr::from_str(ip.as_str()).expect("TODO: Failed to parse vision server ip");
        let socket =
            UdpSocket::bind(format!("{}:{}", ip, port)).expect("Failed to bind the UDP Socket");
        socket
            .join_multicast_v4(&ipv4, &Ipv4Addr::UNSPECIFIED)
            .expect("Error to join multicast group");
        socket
            .set_nonblocking(true)
            .expect("Failed to set non blocking");

        Self {
            socket,
            buffer: [0u8; BUFFER_SIZE],
            tx,
        }
    }

    pub fn run(&mut self) {
        if let Ok(p_size) = self.socket.recv(&mut self.buffer) {
            let packet = T::decode(Cursor::new(&self.buffer[0..p_size]))
                .expect("Error - Decoding the packet");

            self.tx.send(packet).expect("TODO: panic message");
        }
    }
}
