use crate::protocol::{Request, Response};
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::mpsc;

pub struct Store {
    map: HashMap<String, String>,
    receiver: mpsc::Receiver<Request>,
}

impl Store {
    pub fn new(receiver: mpsc::Receiver<Request>) -> Self {
        Store {
            map: HashMap::new(),
            receiver,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while let Some(task) = self.receiver.recv().await {
            match task {
                Request::Get { key, sender } => {
                    let response = self.get(&key);
                    let _ = sender.send(response);
                }
                Request::Set { key, value, sender } => {
                    let response = self.set(key, value);
                    let _ = sender.send(response);
                }
                Request::Delete { key, sender } => {
                    let response = self.delete(key);
                    let _ = sender.send(response);
                }
            }
        }

        Ok(())
    }

    fn get(&self, key: &str) -> Response {
        match self.map.get(key) {
            Some(value) => Response::Get {
                value: value.to_owned(),
            },
            None => Response::Error {
                msg: format!("no key {}", key),
            },
        }
    }

    fn set(&mut self, key: String, value: String) -> Response {
        self.map.insert(key.clone(), value.clone());
        Response::Set { key, value }
    }

    fn delete(&mut self, key: String) -> Response {
        let value = self.map.remove(&key);
        match value {
            Some(value) => Response::Delete { value },
            None => Response::Error {
                msg: format!("no key {}", key),
            },
        }
    }
}
