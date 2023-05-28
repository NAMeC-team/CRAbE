use crabe_io::communication::MulticastUDPReceiver;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;
use std::net::Ipv4Addr;

fn main() {
    let mut vision = MulticastUDPReceiver::new(Ipv4Addr::new(224, 5, 23, 2), 10020)
        .expect("Error to create Vision UDP Receiver");
    loop {
        if let Some(packet) = vision.receive::<SslWrapperPacket>() {
            dbg!(packet);
        }
    }
}
