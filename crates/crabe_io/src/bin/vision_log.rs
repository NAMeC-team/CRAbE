use std::net::Ipv4Addr;
use crabe_io::communication::MulticastUDPReceiver;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;

fn main() {
    let mut vision = MulticastUDPReceiver::new(Ipv4Addr::new(224,5,23,2), 10020)
        .expect("Error to create multicast UDP Receiver");
    loop {
        if let Some(packet) = vision.receive::<SslWrapperPacket>() {
            dbg!(packet);
        }
    }
}
