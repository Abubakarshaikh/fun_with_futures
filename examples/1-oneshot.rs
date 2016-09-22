#![feature(conservative_impl_trait)]

extern crate futures;
extern crate rand;

use std::thread;
use futures::Future;
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
    let (tx_1, rx_1) = futures::oneshot();
    let (tx_2, rx_2) = futures::oneshot();

    thread::spawn(move || {
        println!("first thread starts.");

        let waited_for = sleep_a_little_bit();
        println!("first thread waited for: {}", waited_for);
        tx_1.complete(waited_for);

        println!("first thread ends");
    });

    thread::spawn(move || {
        println!("second thread starts.");

        let waited_for = sleep_a_little_bit();
        println!("second thread waited for: {}", waited_for);
        tx_2.complete(waited_for);

        println!("second thread ends");
    });

    rx_1.join(rx_2).map(|(a, b)| {
        println!("Sum of waiting times was: {}", a + b);
    }).wait().unwrap();
}
