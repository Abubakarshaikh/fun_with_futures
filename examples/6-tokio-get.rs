extern crate futures;
extern crate tokio_core;

use std::net::ToSocketAddrs;
use futures::future::Future;
use tokio_core::io::{read_to_end, write_all};
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;

const DOMAIN: &'static str = "google.com";
const PORT: u16 = 80;

fn main() {
    // Create the event loop that will drive this server.
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Get a socket address from a domain name.
    let socket_addr = (DOMAIN, PORT).to_socket_addrs()
        .map(|iter| iter.collect::<Vec<_>>())
        .unwrap()[0];
    
    // Connect with a handle to the core event loop.
    let response = TcpStream::connect(&socket_addr, &handle)
        .and_then(|socket| {
            // Write a raw GET request onto the socket.
            write_all(socket, format!("\
                GET / HTTP/1.0\r\n\
                Host: {}\r\n\
                \r\n\
            ", DOMAIN))
        }).and_then(|(socket, _request)| {
            // Read the response into a buffer.
            read_to_end(socket, Vec::new())
        });

    // Fire the task off onto the event loop.
    let (_socket, data) = core.run(response).unwrap();

    // Parse the data and output it.
    println!("{}", String::from_utf8(data).unwrap());
}