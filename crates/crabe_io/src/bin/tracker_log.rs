use std::net::Ipv4Addr;
use crabe_io::communication::MulticastUDPReceiver;
use crabe_protocol::protobuf::vision_packet::TrackerWrapperPacket;

fn main() {
    let mut tracker = MulticastUDPReceiver::new(Ipv4Addr::new(224, 5, 23, 2), 10010)
        .expect("Can't create tracker data receiver");
    
    println!("Listening for tracker packts at 224.5.23.2:10010");
    loop {
        if let Some(packet) = tracker.receive::<TrackerWrapperPacket>() {
            println!("{:?}", packet);
        }
    }
}