use crate::protocol::{Request, Response};
use std::error::Error;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use futures::SinkExt;

pub struct Server {
    sender: mpsc::Sender<Request>,
}

impl Server {
    pub fn new(sender: mpsc::Sender<Request>) -> Self {
        Self { sender }
    }

    pub async fn run(self, listener: TcpListener) -> Result<(), Box<dyn Error>> {
        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    let sender = self.sender.clone();

                    tokio::spawn(async move {
                        let mut lines = Framed::new(socket, LinesCodec::new());

                        while let Some(result) = lines.next().await {
                            match result {
                                Ok(line) => {
                                    let response = Self::handle_request(&sender, &line).await;
                                    let response = response.serialize();
                                    if let Err(e) = lines.send(response.as_str()).await {
                                        println!("error on sending response; error = {:?}", e);
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

    async fn handle_request(sender: &mpsc::Sender<Request>, line: &str) -> Response {
        let (request, receiver) = match Request::parse(line) {
            Ok(req) => req,
            Err(e) => return Response::Error { msg: e },
        };

        if let Err(e) = sender.send(request).await {
            return Response::Error { msg: e.to_string() };
        }

        match receiver.await {
            Ok(response) => response,
            Err(e) => Response::Error { msg: e.to_string() },
        }
    }
}
