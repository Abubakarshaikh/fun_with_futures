extern crate futures;
extern crate fun_with_futures;
extern crate futures_cpupool;

use futures::future::{Future, join_all};
use futures_cpupool::Builder;

use fun_with_futures::sleep_a_little_bit;

// Feel free to change me!
const NUM_OF_TASKS: usize = 10;

fn main() {
    // Creates a CpuPool with workers equal to the cores on the machine.
    let pool = Builder::new()
        .create();

    // Create a batch of futures.
    let futures = (0..NUM_OF_TASKS)
        // Tell the pool to run a closure.
        .map(|index| pool.spawn_fn(move || {

            println!("{} --> START", index);
            let waited_for = sleep_a_little_bit();
            println!("{} <-- WAITED {}", index, waited_for);

            // We need to return a result!
            // Why? Futures were implemented with I/O in mind!
            let result: Result<_, ()> = Ok(index);

            result
        })).collect::<Vec<_>>();

    // We wait on all the futures here and see if they came back in order.
    let result = join_all(futures)
        // Check they all came out in the right order.
        .map(|values|
            values.iter()
                .enumerate()
                .all(|(index, &value)| index == value))
        .wait()
        .unwrap();
    println!("Job is done. Values returned in order: {:?}", result)
}
