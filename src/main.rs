mod traditional;

fn main() {
    // traditional
    let mut client = traditional::Client::new();
    client.connect("localhost:8080").unwrap();
    client.get("/users").unwrap();
    client.post("/users", "body").unwrap();
    client.disconnect().unwrap();

    match client.get("/users") {
        Ok(()) => println!("success"),
        Err(e) => println!("runtime error: {e:?}"),
    }
}
