extern crate rand;

use std::thread;
use std::time::Duration;
use rand::distributions::{Range, IndependentSample};

// This function sleeps for a bit then returns how long it slept.
pub fn sleep_a_little_bit() -> u64 {
    let mut generator = rand::thread_rng();
    let possibilities = Range::new(0, 1000);

    let choice = possibilities.ind_sample(&mut generator);

    let a_little_bit = Duration::from_millis(choice);
    thread::sleep(a_little_bit);
    choice
}
