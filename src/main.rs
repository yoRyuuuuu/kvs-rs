use std::{collections::HashMap, error::Error, hash::Hash, net::SocketAddr};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

pub enum Request {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
}

pub enum Response {
    Get { key: String },
    Set { key: String, value: String },
    Delete { value: String },
    Error { msg: String },
}

pub struct Store {
    map: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            map: HashMap::new(),
        }
    }
}

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self, listener: TcpListener) -> Result<(), Box<dyn Error>> {
        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    tokio::spawn(async move {
                        let mut lines = Framed::new(socket, LinesCodec::new());
                        while let Some(result) = lines.next().await {
                            match result {
                                Ok(line) => {
                                    println!("{}", line);
                                    if let Err(e) = Self::request_handle().await {
                                        println!("errror on sending response; error = {:?}", e)
                                    }
                                }
                                Err(e) => {
                                    println!("error on decoding from socket; error = {:?}", e);
                                }
                            }
                        }
                    });
                }
                Err(e) => println!("error accepting socket; error = {:?}", e),
            }
        }
    }

    async fn request_handle() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(socket).await.unwrap();
    let server = Server::new();
    server.run(listener).await
}
