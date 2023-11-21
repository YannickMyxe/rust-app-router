# rust-app-router

A web app router written in rust
Currently only supports adding files to a route

## Setup

First create an address with an **IP** and a **Port**

```rust
let addr = Address::new( & "127.0.0.1", 7878);
let mut listener = Listener::from_address(addr);
```

### Add a route

To add a route add the following code

```rust
listener.add_route("/", Rc::new(Handle::new(ResponseCode::Ok, "html/homepage.html")))
```

### Listen on the address

Now run the following code to listen to upcomming requests

```rust
listener.listen();
```

## Run the code with

```shell
cargo run
```

## Exit the program

Currently, the router has no graceful way to close out of the listen command,
you should use `CTRL + C` to cancel the last command

