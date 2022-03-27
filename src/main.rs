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

fn main() {}
