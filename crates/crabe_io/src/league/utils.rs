use std::any::type_name;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::thread;
use log::error;
use std::thread::JoinHandle;
use crate::communication::MulticastUDPReceiver;

type ThreadReceiverData<P> = (Receiver<P>, JoinHandle<()>, Arc<AtomicBool>);

/// Creates a new multicast UDP receiver receiving packets of type P
/// in a separate thread.
pub fn threaded_receiver<P>(ip_addr: &str, port: u16) -> ThreadReceiverData<P>
where
    P: Default + prost::Message + 'static
{
    let (tx, rx) = mpsc::channel::<P>();
    let ipv4 = Ipv4Addr::from_str(ip_addr)
        .expect("Failed to create an ipv4 address with the ip");
    let mut udp_receiver =
        MulticastUDPReceiver::new(ipv4, port).expect("Failed to create vision receiver");

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    
    let handle = thread::spawn(move || {
        while running_clone.load(Ordering::Relaxed) {
            if let Some(packet) = udp_receiver.receive() {
                if let Err(e) = tx.send(packet) {
                    error!("Error sending {:} packet: {:?}", type_name::<P>(), e);
                }
            }
        }
    });
    
    (rx, handle, running)
}