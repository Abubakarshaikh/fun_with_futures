#![feature(conservative_impl_trait)]

extern crate futures;
extern crate rand;

use std::thread;
use futures::{Future, collect};
use std::time::Duration;
use rand::distributions::{Range, IndependentSample};

// This function sleeps for a bit then returns how long it slept.
fn sleep_a_little_bit() -> u64 {
    let mut generator = rand::thread_rng();
    let possibilities = Range::new(0, 1000);

    let choice = possibilities.ind_sample(&mut generator);

    let a_little_bit = Duration::from_millis(choice);
    thread::sleep(a_little_bit);
    choice
}

fn main() {
    // We'll create a set to add all of the recievers to.
    let mut rx_vec = Vec::new();

    // Next we'll spawn up a bunch of threads doing 'something' for a bit then sending a value.
    for index in 0..100 {
        // Here we create a future, this is a `oneshot` value which is consumed after use.
        let (tx, rx) = futures::oneshot();
        // Add the reciever to the vector we created earlier so we can collect on it.
        rx_vec.push(rx);

        // Spawning up a thread means things won't be executed sequentially, so this will actually
        // behave like a future value, so we can actually see how they work.
        thread::spawn(move || {
            println!("{} --> START.", index);

            let waited_for = sleep_a_little_bit();
            println!("{} --- WAITED {}", index, waited_for);
            // Here we send back the value (and consume the sender).
            tx.complete(waited_for);

            println!("{} <-- END", index);
        });
    }

    // `collect` lets us pull through a series of Futures.
    let result = collect(rx_vec)
        // Block until they all are resolved.
        .wait()
        // Sum them up.
        .map(|values| values.iter().fold(0, |acc, val| acc + val))
        // And finally get the value out.
        .unwrap();

    // Now what was the sum of all the waiting times?
    println!("SUM {:?}", result);
}
