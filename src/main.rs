use protocol::Request;
use server::Server;
use std::{error::Error, net::SocketAddr};
use store::Store;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

mod protocol;
mod server;
mod store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(socket).await.unwrap();
    let (sender, receiver) = mpsc::channel::<Request>(32);
    let server = Server::new(sender);
    let mut store = Store::new(receiver);

    tokio::spawn(async move {
        if let Err(e) = store.run().await {
            println!("{}", e);
        }
    });

    server.run(listener).await
}
