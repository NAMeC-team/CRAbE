use crabe_io::network::multicast_receiver::MulticastReceiver;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;

fn main() {
    let mut vision = MulticastReceiver::<SslWrapperPacket>::new("224.5.23.2".to_string(), 10020);
    loop {
        let packet = vision.run();
        dbg!(packet);
    }
}