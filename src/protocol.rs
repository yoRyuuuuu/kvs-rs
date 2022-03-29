use tokio::sync::oneshot;

pub enum Request {
    Get {
        key: String,
        sender: oneshot::Sender<Response>,
    },
    Set {
        key: String,
        value: String,
        sender: oneshot::Sender<Response>,
    },
    Delete {
        key: String,
        sender: oneshot::Sender<Response>,
    },
}

pub enum Response {
    Get { value: String },
    Set { key: String, value: String },
    Delete { value: String },
    Error { msg: String },
}

impl Response {
    pub fn serialize(&self) -> String {
        match self {
            Response::Get { ref value } => format!("{}", value),
            Response::Set { ref key, ref value } => format!("{} = {}", key, value),
            Response::Delete { ref value } => format!("{}", value),
            Response::Error { ref msg } => format!("error: {}", msg),
        }
    }
}

impl Request {
    pub fn parse(input: &str) -> Result<(Request, oneshot::Receiver<Response>), String> {
        let mut parts = input.splitn(3, ' ');
        let (sender, receiver) = oneshot::channel();
        let request = match parts.next() {
            Some("GET") => {
                let key = parts.next().ok_or("GET must be followed by a key")?;
                if parts.next().is_some() {
                    return Err("GET's key must not be followed by anything".into());
                }

                Request::Get {
                    key: key.to_string(),
                    sender,
                }
            }
            Some("SET") => {
                let key = match parts.next() {
                    Some(value) => value,
                    None => return Err("SET must be followed by a key".into()),
                };
                let value = match parts.next() {
                    Some(value) => value,
                    None => return Err("SET needs a value".into()),
                };

                Request::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                    sender,
                }
            }
            Some("DELETE") => {
                let key = match parts.next() {
                    Some(value) => value,
                    None => return Err("DELETE must be followed by a key".into()),
                };

                Request::Delete {
                    key: key.to_string(),
                    sender,
                }
            }
            _ => return Err("empty input".into()),
        };

        Ok((request, receiver))
    }
}
