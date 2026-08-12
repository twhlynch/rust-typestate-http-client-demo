#[derive(Debug, PartialEq)]
pub enum ClientError {
    NotConnected,
    AlreadyConnected,
}

pub struct Client {
    connected: bool,
    addr: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        return Self {
            connected: false,
            addr: None,
        };
    }

    pub fn connect(&mut self, addr: &str) -> Result<(), ClientError> {
        if self.connected {
            return Err(ClientError::AlreadyConnected);
        }

        println!("Connecting to {addr}...");

        self.connected = true;
        self.addr = Some(addr.to_string());
        return Ok(());
    }

    pub fn get(&mut self, path: &str) -> Result<(), ClientError> {
        if !self.connected {
            return Err(ClientError::NotConnected);
        }

        println!("GET {path}");

        return Ok(());
    }

    pub fn post(&mut self, path: &str, body: &str) -> Result<(), ClientError> {
        if !self.connected {
            return Err(ClientError::NotConnected);
        }

        println!("POST {path} {body}");

        return Ok(());
    }

    pub fn disconnect(&mut self) -> Result<(), ClientError> {
        if !self.connected {
            return Err(ClientError::NotConnected);
        }

        println!("Disconnected from {}.", self.addr.as_ref().unwrap());

        self.connected = false;
        self.addr = None;
        return Ok(());
    }
}
