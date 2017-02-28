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

enum ExampleRequest {
    Get { url: String },
    Post { url: String, content: String },
}

enum ExampleResponse {
    NotFound,
    Ok { content: String },
}

// Codecs can have state, but this one doesn't.
struct ExampleCodec;
impl Codec for ExampleCodec {
    // Associated types which define the data taken/produced by the codec.
    type In = ExampleRequest;
    type Out = ExampleResponse;

    // Returns `Ok(Some(In))` if there is a frame, `Ok(None)` if it needs more data.
    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        let content_so_far = String::from_utf8_lossy(buf.as_slice())
            .to_mut()
            .clone();

        // Request headers/content in HTTP 1.1 are split by double newline.
        match content_so_far.find("\r\n\r\n") {
            Some(i) => {
                // Drain from the buffer (this is important!)
                let mut headers = {
                    let tmp = buf.drain_to(i);
                    String::from_utf8_lossy(tmp.as_slice())
                        .to_mut()
                        .clone()
                };
                buf.drain_to(4); // Also remove the '\r\n\r\n'.

                // Get the method and drain.
                let method = headers.find(" ")
                    .map(|len| headers.drain(..len).collect::<String>());

                headers.drain(..1); // Get rid of the space.

                // Since the method was drained we can do it again to get the url.
                let url = headers.find(" ")
                    .map(|len| headers.drain(..len).collect::<String>())
                    .unwrap_or_default();

                // The content of a POST.
                let content = {
                    let remaining = buf.len();
                    let tmp = buf.drain_to(remaining);
                    String::from_utf8_lossy(tmp.as_slice())
                        .to_mut()
                        .clone()
                };

                match method {
                    Some(ref method) if method == "GET" => {
                        Ok(Some(ExampleRequest::Get { url: url }))
                    },
                    Some(ref method) if method == "POST" => {
                        Ok(Some(ExampleRequest::Post { url: url, content: content }))
                    },
                    _ => Err(io::Error::new(io::ErrorKind::Other, "invalid"))
                }
            },
            None => Ok(None),
        }
    }

    // Produces a frame.
    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        match msg {
            ExampleResponse::NotFound => {
                buf.extend(b"HTTP/1.1 404 Not Found\r\n\r\n");
            },
            ExampleResponse::Ok { content: v } =>  {
                buf.extend(b"HTTP/1.1 200 Ok\r\n");
                buf.extend(v.as_bytes());
                buf.extend(b"\r\n");
            }
        }
        Ok(())
    }
}

// This is a silly function to workaround for testing.
fn mapper(req: ExampleRequest) -> ExampleResponse {
    match req {
        ExampleRequest::Get { url: url } => {
            ExampleResponse::NotFound
        },
        ExampleRequest::Post { url: url, content: content } => {
            ExampleResponse::Ok { content: content }
        }
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
            let (writer, reader) = stream.framed(ExampleCodec).split();
            // Here we just sink any data recieved directly back into the socket.
            reader.map(mapper).forward(writer)
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