use std::rc::Rc;
use rust_router::{Address, Listener, };
use rust_router::fs_util::read_dir;
use rust_router::routes::{Handle, ResponseCode};

fn main() {
    let version = "0.1.0";
    println!("Welcome to rust-router {}", version);
    let addr = Address::new(&"127.0.0.1", 7878);
    let mut listener = Listener::from_address(addr);

    println!("[Server] created on  {}", listener.clone().get());
    println!("Loading dir:");
    read_dir("html/");

    {
        let handle = Rc::new(Handle::new(ResponseCode::Ok, "html/homepage.html"));
        listener.add_route("/", handle);
        listener.add_route("/about", Rc::new(Handle::new(ResponseCode::Ok, "html/about.html")))
    } // Mutable borrow goes out of scope here

    listener.listen();
}

