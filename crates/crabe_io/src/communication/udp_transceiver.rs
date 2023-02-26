use crate::constant::BUFFER_SIZE;
use log::error;
use std::io::Cursor;
use std::net::UdpSocket;

/// A struct that provides bidirectional communication over UDP.
pub struct UDPTransceiver {
    /// The underlying non-blocking UDP socket that sends and receives data.
    socket: UdpSocket,
    /// A buffer that is used to receive data from the socket without allocating new heap memory.
    buffer: [u8; BUFFER_SIZE],
}

impl UDPTransceiver {
    /// Creates a new non-blocking `UDPTransceiver` that listens on the specified IP address and port number.
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address of the UDP transceiver as a string slice.
    /// * `port` - The port number of the UDP transceiver.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new non-blocking `UDPTransceiver` that is ready to send and receive data if the operation is successful, or an `std::io::Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function will return an `std::io::Error` if there is an error while binding and connect the socket or setting the socket to non-blocking mode.
    ///
    /// # Example
    ///
    /// ```
    /// use crabe_io::communication::UDPTransceiver;
    ///
    /// let transceiver = UDPTransceiver::new("127.0.0.1", 10301).expect("Failed to create UDPTransceiver");
    /// ```
    ///
    /// This example creates a new non-blocking `UDPTransceiver` that listens on IP address 127.0.0.1 and port 10301, which is the default grSim control port for the blue team.
    pub fn new(ip: &str, port: u32) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_nonblocking(true)?;
        socket.connect(format!("{}:{}", ip, port))?;
        let buffer = [0u8; BUFFER_SIZE];

        Ok(Self { socket, buffer })
    }

    /// Sends a `T` packet over the underlying UDP socket.
    ///
    /// # Arguments
    ///
    /// * `packet`: The packet to send. It should be a struct generated by protobuf files using `prost` that implement the `prost::Message` and `Default` traits.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type of the packet to send. It must implement the `prost::Message` and `Default` traits. It's a struct generated by protobuf files using `prost`.
    pub fn send<T: prost::Message + Default>(&self, packet: T) {
        let mut buf = Vec::with_capacity(packet.encoded_len());
        if let Err(e) = packet.encode(&mut buf) {
            error!("Failed to encode the packet: {}", e);
            return;
        }

        let data = &buf[0..packet.encoded_len()];
        if let Err(e) = self.socket.send(data) {
            error!("Failed to send data: {}", e);
        }
    }

    /// Attempts to receive a packet of type `U` from the socket, and decodes it using `prost`.
    ///
    /// # Type Parameters
    ///
    /// * `U`: The type of packet to decode. It must implement the `prost::Message` and `Default` traits, and should be a struct generated by protobuf files using `prost`.
    ///
    /// # Returns
    ///
    /// An `Option` that contains the decoded packet if the receive operation is successful and decoding is successful, or `None` otherwise.
    pub fn receive<U: prost::Message + Default>(&mut self) -> Option<U> {
        match self.socket.recv(&mut self.buffer) {
            Ok(p_size) => match U::decode(Cursor::new(&self.buffer[0..p_size])) {
                Ok(packet) => Some(packet),
                Err(e) => {
                    error!("Decoding of the received packet failed: {}", e);
                    None
                }
            },
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    None
                } else {
                    error!("Failed to receive data from the socket: {}", e);
                    None
                }
            }
        }
    }
}
