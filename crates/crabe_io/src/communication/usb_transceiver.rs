use crate::constant::BUFFER_SIZE;
use log::{debug, error};
use serialport::SerialPort;
use std::time::Duration;

pub struct UsbTransceiver {
    port: Box<dyn SerialPort>,
    buffer: [u8; BUFFER_SIZE],
}

impl UsbTransceiver {
    pub fn new(port: &str, baud: u32) -> Result<Self, serialport::Error> {
        let port = serialport::new(port, baud)
            .timeout(Duration::from_millis(1))
            .open()?;

        let buffer = [0u8; BUFFER_SIZE];

        Ok(Self { port, buffer })
    }

    pub fn send<T: prost::Message + Default>(&mut self, packet: T) {
        let mut buf = Vec::new();
        buf.reserve(packet.encoded_len() + 1);
        buf.push(packet.encoded_len() as u8);
        packet.encode(&mut buf).unwrap();

        match self.port.write(&buf[0..packet.encoded_len() + 1]) {
            Ok(_v) => {
                debug!("sent: {:?}", packet);
            }
            Err(e) => {
                error!("send error: {}", e);
            }
        }
    }
}
