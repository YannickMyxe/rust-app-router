pub mod fs_util;
pub mod routes;

use std::fmt::{Display, Formatter};
use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};
use std::collections::HashMap;
use std::rc::Rc;
use crate::routes::{Handle, ResponseCode, Routes};

pub struct Address<'a> {
    pub ip: &'a str,
    pub port: u16,
}

impl Display for Address<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

impl Address<'_> {
    pub fn new(ip: &str, port: u16) -> Address {
        return Address {
            ip: ip.clone(),
            port,
        };
    }

    pub fn get(&self) -> String {
        return format!("{}:{}", self.ip, self.port);
    }
}

pub struct Listener<'a> {
    address: Address<'a>,
    routes: Routes<'a>,
}

impl<'a> Listener<'a> {
    pub fn new(ip: &str, port: u16) -> Listener {
        return Listener {
            address: Address::new(ip, port),
            routes: Routes::new(),
        };
    }

    pub fn add_route(&mut self, route: &'a str, handle: Rc<Handle<'a>>) {
        self.routes.add(route.to_string(), handle);
    }

    pub fn get_routes(&self) -> HashMap<String, Rc<Handle<'a>>> {
        return self.routes.routes_map.clone();
    }

    pub fn from_address(addr: Address) -> Listener {
        return Listener::new(addr.ip, addr.port);
    }

    pub fn get(&self) -> String {
        return self.address.get().clone();
    }

    pub fn bind(&self) -> std::io::Result<TcpListener> {
        return TcpListener::bind(self.address.get());
    }

    pub fn listen(&self) {
        match self.bind() {
            Ok(listen) => {
                for stream in listen.incoming() {
                    let stream = stream.unwrap();
                    println!(
                        "[Client]: Connected on {}",
                        stream.peer_addr().expect("Could not get peer address")
                    );
                    self.handle_connection(stream);
                }
            }
            Err(e) => {
                eprintln!("Could not successfully bind to {}: => {}", self.get(), e);
            }
        }
    }

    pub fn response(&self, mut stream: TcpStream, handle: &Handle) {
        let contents = fs::read_to_string(handle.file).expect(format!("Not able to read file contents {}", handle.file).as_str());
        let length = contents.len();

        let response =
            format!("{}\r\nContent-Length: {length}\r\n\r\n{contents}", handle.code.as_str());

        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let mut http_request = buf_reader.lines();

        let request_line = http_request.next().unwrap().unwrap();

        if request_line.starts_with("GET") && request_line.ends_with("HTTP/1.1") {
            let route = request_line
                .strip_prefix("GET")
                .unwrap()
                .strip_suffix("HTTP/1.1")
                .unwrap()
                .trim();

            println!("Route: {{{route}}}");
            match self.routes.handle_request(route) {
                Some(handle) => {
                    self.response(stream, handle);
                },
                None => {
                    match http_request.next() {
                        None => {}
                        Some(value) => {
                            let host = value.unwrap().strip_prefix("Host: ").unwrap().to_owned();
                            if host.eq(self.get().as_str()) {
                                self.response(stream, &Handle::new(ResponseCode::Ok, format!("./html{}", route.clone()).as_str()))
                            }
                        }
                    }
                },
            }
        }
    }

}

impl Clone for Listener<'_> {
    fn clone(&self) -> Self {
        return Listener::new(self.address.ip, self.address.port);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let addr = Address {
            ip: "127.0.0.1",
            port: 7878,
        };
        let listener = Listener::new(addr.ip, addr.port).bind();
        match listener {
            Ok(_) => {
                println!("Bound to {addr} successfully");
            }
            Err(e) => {
                panic!("Could not bind to {addr} => {e}");
            }
        }
    }
}
