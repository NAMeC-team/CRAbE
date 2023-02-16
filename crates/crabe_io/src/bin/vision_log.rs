use crabe_io::network::multicast_udp_receiver::MulticastUDPReceiver;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;

fn main() {
    let mut vision = MulticastUDPReceiver::new("224.5.23.2", 10020);
    loop {
        if let Some(packet) = vision.receive::<SslWrapperPacket>() {
            dbg!(packet);
        }
    }
}
