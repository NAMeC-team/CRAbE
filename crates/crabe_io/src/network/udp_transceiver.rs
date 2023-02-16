use crate::constants::BUFFER_SIZE;
use log::error;
use std::io::Cursor;
use std::net::UdpSocket;

/// A struct that provides bidirectional communication over UDP.
pub struct UDPTransceiver {
    /// The underlying UDP socket that sends and receives data.
    socket: UdpSocket,
    /// A buffer that is used to receive data from the socket without allocating new heap memory.
    buffer: [u8; BUFFER_SIZE],
}

impl UDPTransceiver {
    /// Creates a new UDP transceiver that listens on the specified IP address and port number.
    ///
    /// # Arguments
    ///
    /// * `ip`: The IP address of the UDP transceiver as a string slice.
    /// * `port`: The port number of the UDP transceiver.
    ///
    /// # Returns
    ///
    /// A new `UDPTransceiver` that is ready to send and receive data.
    ///
    /// # Example
    ///
    /// ```
    /// use crabe_io::network::UDPTransceiver;
    ///
    /// let transceiver = UDPTransceiver::new("127.0.0.1", 10301);
    /// ```
    ///
    /// This example creates a new `UDPTransceiver` that listens on IP address 127.0.0.1 and port 10301, which is the default grSim control port for the blue team.
    pub fn new(ip: &str, port: u32) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind the UDP Socket");

        socket
            .set_nonblocking(true)
            .expect("Failed to set socket to non-blocking mode");
        socket
            .connect(format!("{}:{}", ip, port)) // TODO : Put some ip
            .expect("connect function failed");

        Self {
            socket,
            buffer: [0u8; BUFFER_SIZE],
        }
    }

    /// TODO:
    pub fn send<T: prost::Message + Default>(&self, packet: T) {
        let mut buf = Vec::new();
        buf.reserve(packet.encoded_len());
        if let Err(e) = packet.encode(&mut buf) {
            error!("Cannot encode the packet in the buffer : {}", e)
        }

        if let Err(e) = self.socket.send(&buf[0..packet.encoded_len()]) {
            error!("Couldn't send data : {}", e);
        }
    }

    /// TODO:
    pub fn receive<T: prost::Message + Default>(&mut self) -> Option<T> {
        match self.socket.recv(&mut self.buffer) {
            Ok(p_size) => match T::decode(Cursor::new(&self.buffer[0..p_size])) {
                Ok(packet) => Some(packet),
                Err(e) => {
                    error!("Receive some packets but decoding message failed : {}", e);
                    None
                }
            },
            Err(_) => None,
        }
    }
}
