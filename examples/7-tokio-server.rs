extern crate futures;
extern crate tokio_core;

use std::net::SocketAddr;
use futures::future::Future;
use futures::Stream;
use tokio_core::io::{read_to_end, write_all};
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

const LISTEN_TO: &'static str = "0.0.0.0:8080";

fn main() {
    // Create the event loop that will drive this server.
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Get a SocketAddr
    let socket: SocketAddr = LISTEN_TO.parse()
        .unwrap();
    
    // Start listening.
    let listener = TcpListener::bind(&socket, &handle)
        .unwrap();
    
    // For each incoming connection...
    let server = listener.incoming().for_each(|(stream, _client_addr)| {
        // Spawn the task on the reactor.
        handle.spawn_fn(|| {
            // Read until EOF
            read_to_end(stream, Vec::new())
                .and_then(|(socket, data)| {
                    // Write the recieved data.
                    write_all(socket, data)
                })
                // Errors have to be handled internally.
                .map(|_| ())
                .map_err(|_| ())
        });

        Ok(()) // keep accepting connections
    });

    // Run the reactor.
    core.run(server).unwrap();
}
// Throw some data at it with `echo "Test" | nc 127.0.0.1 8080`