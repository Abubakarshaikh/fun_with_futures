extern crate futures;
extern crate rand;

use std::thread;
use futures::{Future, finished};
use futures::stream::{self, Stream};
use std::time::Duration;
use rand::distributions::{Range, IndependentSample};

fn sleep_a_little_bit() -> u64 {
    let mut generator = rand::thread_rng();
    let possibilities = Range::new(0, 1000);

    let choice = possibilities.ind_sample(&mut generator);

    let a_little_bit = Duration::from_millis(choice);
    thread::sleep(a_little_bit);
    choice
}

fn main() {
    let (mut tx, rx) = stream::channel::<u64, u64>();

    thread::spawn(move || {
        println!("first thread starts.");
        for index in 0..10 {
            let waited_for = sleep_a_little_bit();
            println!("index {} waited for: {}", index, waited_for);
            match tx.send(Ok(waited_for)).wait() {
                Ok(new_tx) => tx = new_tx,
                Err(_) => panic!("Oh no!"),
            }
        }
        println!("first thread ends");
    });

    let sum = rx.fold(0, |acc, val| { finished::<u64, u64>(acc + val) }).wait();
    println!("sum is {}", sum.unwrap());
}
