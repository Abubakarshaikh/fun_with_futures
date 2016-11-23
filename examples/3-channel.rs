extern crate futures;
extern crate fun_with_futures;

use std::thread;
use futures::future::{Future, ok};
use futures::stream::Stream;
use futures::sync::mpsc;
use futures::Sink;

use fun_with_futures::sleep_a_little_bit;

const BUFFER_SIZE: usize = 10;

fn main() {
    // A channel represents a stream and will yeild a series of futures.

    // We're using a bounded channel here with a limited size.
    let (mut tx, rx) = mpsc::channel(BUFFER_SIZE);

    thread::spawn(move || {
        println!("--> START THREAD");
        // We'll have the stream produce a series of values.
        for _ in 0..10 {

            let waited_for = sleep_a_little_bit();
            println!("--- THREAD WAITED {}", waited_for);

            // When we `.send()` a value it consumes the sender. Returning
            // a 'new' sender which we have to handle. In this case we just
            // re-assign.
            match tx.send(waited_for).wait() {
                Ok(new_tx) => tx = new_tx,
                Err(_) => panic!("Oh no!"),
            }

        }
        println!("<-- END THREAD");
        // Here the stream is dropped.
    });

    // We can `.fold()` like we would an iterator. In fact we can do many
    // things like we would an iterator.
    let sum = rx.fold(0, |acc, val| {
            // Notice when we run that this is happening after each item of
            // the stream resolves, like an iterator.
            println!("--- FOLDING {} INTO {}", val, acc);
            // `ok` is a simple way to say "Yes this worked." 
            ok(acc + val)
        })
        .wait()
        .unwrap();
    println!("SUM {}", sum);
}
