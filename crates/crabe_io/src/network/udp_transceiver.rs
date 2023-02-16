use std::io::Cursor;
use std::net::UdpSocket;
use crate::constants::BUFFER_SIZE;


/// TODO:
pub struct UDPTransceiver {
    socket: UdpSocket,
    buffer: [u8; BUFFER_SIZE],
}

/// TODO:
impl UDPTransceiver {
    /// TODO:
    pub fn new(port: u32) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind the UDP Socket");

        socket
            .set_nonblocking(true)
            .expect("Failed to set socket to non-blocking mode");
        socket
            .connect(format!("127.0.0.1:{}", port)) // TODO : Put some ip
            .expect("connect function failed");

        Self {
            socket,
            buffer: [0u8; BUFFER_SIZE],
        }
    }

    pub fn send<T: prost::Message+ Default>(&self, packet: T) {
        let mut buf = Vec::new();
        buf.reserve(packet.encoded_len());
        if let Err(e) = packet.encode(&mut buf) {
            println!("{}", e)
        }

        if let Err(e) = self.socket.send(&buf[0..packet.encoded_len()]) {
            println!("Couldn't send data {}", e);
        }
    }

    pub fn receive<T: prost::Message + Default>(&mut self) -> Option<T> {
        match self.socket.recv(&mut self.buffer) {
            Ok(p_size) => {
                match T::decode(Cursor::new(&self.buffer[0..p_size])) {
                    Ok(packet) => {
                        Some(packet)
                    }
                    Err(e) => {
                        println!("{}", e);
                        None
                    }
                }
            }
            Err(_e) => {
                // error!("couldn't recv from socket, err: {}", e);
                None
            }
        }
    }
}
