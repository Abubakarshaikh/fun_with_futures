extern crate futures;
extern crate tokio_core;

use std::net::SocketAddr;
use futures::future::Future;
use futures::Stream;
use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf, Io};
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

const LISTEN_TO: &'static str = "0.0.0.0:8080";

// Codecs can have state, but this one doesn't.
struct EchoCodec;
impl Codec for EchoCodec {
    // Associated types which define the data taken/produced by the codec.
    type In = String;
    type Out = String;

    // Returns `Ok(Some(In))` if there is a frame, `Ok(None)` if it needs more data.
    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        match buf.as_slice().iter().position(|&b| b == b'\n') {
            Some(i) => {
                // Drain from the buffer (this is important!)
                let line = buf.drain_to(i);

                // Also remove the '\n'.
                buf.drain_to(1);

                // Turn this data into a UTF string and return it in a Frame.
                match str::from_utf8(line.as_slice()) {
                    Ok(s) => Ok(Some(s.to_string())),
                    Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                                "invalid UTF-8")),
                }
            },
            None => Ok(None),
        }
    }

    // Produces a frame.
    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        // Add the necessary newline.
        buf.push(b'\n');
        Ok(())
    }
}

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
            // Using our codec, split the in/out into frames.
            let (writer, reader) = stream.framed(EchoCodec).split();
            // Here we just sink any data recieved directly back into the socket.
            reader.forward(writer)
                // We need to handle errors internally.
                .map(|_| ())
                .map_err(|_| ())
        });

        Ok(()) // keep accepting connections
    });

    // Run the reactor.
    core.run(server).unwrap();
}
// Throw some data at it with `echo "Test" | nc 127.0.0.1 8080`