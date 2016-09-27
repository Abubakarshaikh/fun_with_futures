extern crate futures;
extern crate fun_with_futures;

use std::thread;
use futures::{Future, finished};
use futures::stream::{self, Stream};

use fun_with_futures::sleep_a_little_bit;

fn main() {
    let (mut tx, rx) = stream::channel::<u64, u64>();

    thread::spawn(move || {
        println!("--> START");
        for _ in 0..10 {
            let waited_for = sleep_a_little_bit();
            println!("--- WAITED {}", waited_for);
            match tx.send(Ok(waited_for)).wait() {
                Ok(new_tx) => tx = new_tx,
                Err(_) => panic!("Oh no!"),
            }
        }
        println!("<-- END");
    });

    let sum = rx.fold(0, |acc, val| {
        println!("--- FOLDING {} INTO {}", val, acc);
        finished::<u64, u64>(acc + val)
    }).wait();
    println!("SUM {}", sum.unwrap());
}
