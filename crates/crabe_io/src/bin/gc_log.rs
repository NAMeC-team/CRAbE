use crabe_io::communication::MulticastUDPReceiver;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use std::net::Ipv4Addr;

fn main() {
    let mut vision = MulticastUDPReceiver::new(Ipv4Addr::new(255, 255, 255, 255), 10003)
        .expect("Error to create GameController UDP Receiver");
    loop {
        if let Some(packet) = vision.receive::<Referee>() {
            dbg!(packet);
        }
    }
}
