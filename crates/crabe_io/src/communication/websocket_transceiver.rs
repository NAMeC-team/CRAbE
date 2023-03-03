use std::error::Error;
use flume::{Receiver, Sender, unbounded};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::net::SocketAddr;
use log::{error, info, log};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio::task::JoinSet;
use tungstenite::Message;

pub struct WebSocketTask<RX, TX> {
    addr: SocketAddr,
    rx: Receiver<TX>,
    tx: Sender<RX>,
    runtime: Runtime,
}

impl<RX: Send + DeserializeOwned, TX: Send + Serialize> WebSocketTask<RX, TX> {
    fn new(addr: SocketAddr, rx: Receiver<TX>, tx: Sender<RX>) -> Self {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        Self {
            addr,
            rx,
            tx,
            runtime,
        }
    }

    async fn handle_connection(
        raw_stream: TcpStream,
        addr: SocketAddr,
        rx: Receiver<TX>,
        tx: Sender<RX>,
    ) -> Result<(), tungstenite::Error>{
        info!("Incoming TCP connection from: {}", addr);
        let ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await?;
        info!("WebSocket connection established: {}", addr);

        let (outgoing, incoming) = ws_stream.split();
        let incoming_fut = incoming
            .filter_map(|x| async move {
                if let Ok(Message::Text(m)) = x {
                    if let Ok(req) = serde_json::from_str::<RX>(&m) {
                        return Some(Ok(req));
                    }
                }

                None
            })
            .forward(tx.sink());
        let outgoing_fut = rx
            .stream()
            .map(|m| Ok(Message::text(serde_json::to_string(&m).unwrap())))
            .forward(outgoing);
        let (_incoming_res, _outgoing_res) = tokio::join!(incoming_fut, outgoing_fut);

        info!("{} disconnected", &addr);

        Ok(())
    }
}

impl<RX: Send + DeserializeOwned + 'static, TX: Send + Serialize + 'static> WebSocketTask<RX, TX>
{
    fn run(&mut self, cancellation: Receiver<()>) {
        let future = async {
            let try_socket = TcpListener::bind(&self.addr).await;
            let listener = try_socket.expect("Failed to bind");
            println!("Listening on: {}", self.addr);
            let receive = async {
                let mut join_set = JoinSet::new();
                while let Ok((stream, addr)) = listener.accept().await {
                    join_set.spawn(
                    Self::handle_connection(
                        stream,
                        addr,
                        self.rx.clone(),
                        self.tx.clone(),
                    ));
                }

                while let Some(res) = join_set.join_next().await {
                    if let Err(join_error) = res {
                        error!("Join error: #{join_error}")
                    } else if let Ok(Err(connection_error)) = res {
                        error!("Connection error: #{connection_error}")
                    }
                }
            };

            tokio::select! {
                _ = receive => {}
                _ = cancellation.recv_async() => {}
            };
        };
        self.runtime.block_on(future);
    }
}

pub struct WebSocketTransceiver<RX, TX> {
    rx: Receiver<RX>,
    tx: Sender<TX>
}

impl<RX: Send, TX> WebSocketTransceiver<RX, TX> {
    fn send(&mut self, msg: TX){
        self.tx.send(msg).unwrap_or_else(|x| error!("Send error: #{x}"));
    }

    fn receive(&mut self) -> Option<RX> {
        self.rx.recv().ok()
    }

    fn new(rx: Receiver<RX>, tx: Sender<TX>) -> Self {
        Self {
            rx,
            tx,
        }
    }
}

pub fn new_websocket<RX: Send + DeserializeOwned, TX: Send + Serialize>(addr: SocketAddr) -> (WebSocketTask<RX, TX>, WebSocketTransceiver<RX, TX>){
    let (task_tx, transceiver_rx) = unbounded();
    let (transceiver_tx, task_rx) = unbounded();

    let task = WebSocketTask::new(addr, task_rx, task_tx);
    let transceiver = WebSocketTransceiver::new(transceiver_rx, transceiver_tx);
    return (task, transceiver);
}