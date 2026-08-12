pub struct Disconnected;
pub struct Connected;

pub struct Client<S> {
    state: std::marker::PhantomData<S>,
    addr: Option<String>,
}

impl Client<Disconnected> {
    pub fn new() -> Self {
        return Self {
            state: std::marker::PhantomData,
            addr: None,
        };
    }

    pub fn connect(self, addr: &str) -> Client<Connected> {
        println!("Connecting to {addr}...");

        return Client {
            state: std::marker::PhantomData,
            addr: Some(addr.to_string()),
        };
    }
}

impl Client<Connected> {
    pub fn get(&self, path: &str) {
        println!("GET {path}");
    }

    pub fn post(&self, path: &str, body: &str) {
        println!("POST {path} {body}");
    }

    pub fn disconnect(self) -> Client<Disconnected> {
        println!("Disconnected from {}.", self.addr.as_ref().unwrap());

        return Client::new();
    }
}
