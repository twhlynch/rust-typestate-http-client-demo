mod traditional;
mod typestate;

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

    // typestate
    let typestate_client = typestate::Client::new()
        .connect("localhost:8080")
        .disconnect()
        .connect("hi")
        .disconnect()
        .connect("");

    typestate_client.get("/users");
    typestate_client.post("/users", "body");

    let _typestate_disconnected = typestate_client.disconnect();
}
