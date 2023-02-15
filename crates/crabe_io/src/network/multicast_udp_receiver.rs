use std::io::Cursor;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use crate::constants::BUFFER_SIZE;

/// TODO:
pub struct MulticastUDPReceiver {
    socket: UdpSocket,
    buffer: [u8; BUFFER_SIZE],
}

/// TODO:
impl MulticastUDPReceiver {
    /// Create a new Multicast UDP Receiver TODO
    /// # Arguments
    ///
    /// * `ip`: The ip address of the Multicast UDP Receiver
    /// * `port`: The port of of the Multicast UDP Receiver
    ///
    /// # Returns
    ///
    /// A Multicast UDP Receiver that join the multicast with the ipv4 address.
    pub fn new(ip: String, port: u32) -> Self {
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
        }
    }

    /// TODO:
    pub fn receive<T: prost::Message + Default> (&mut self) -> Option<T> {
        if let Ok(p_size) = self.socket.recv(&mut self.buffer) {
            let packet = T::decode(Cursor::new(&self.buffer[0..p_size]))
                .expect("Error - Decoding the packet");
            Some(packet)
        } else {
            None
        }
    }
}
