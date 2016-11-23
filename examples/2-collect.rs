extern crate futures;
extern crate fun_with_futures;

use std::thread;
use futures::{Future, collect};

use fun_with_futures::sleep_a_little_bit;

fn main() {
    // We'll create a set to add all of the recievers to.
    let mut rx_vec = Vec::new();

    // Next we'll spawn up a bunch of threads doing 'something' for a bit then sending a value.
    for index in 0..1000 {
        // Here we create a future, this is a `oneshot` value which is consumed after use.
        let (tx, rx) = futures::oneshot();
        // Add the reciever to the vector we created earlier so we can collect on it.
        rx_vec.push(rx);

        // Spawning up a thread means things won't be executed sequentially, so this will actually
        // behave like a future value, so we can actually see how they work.
        thread::spawn(move || {
            println!("{} --> START", index);

            let waited_for = sleep_a_little_bit();
            println!("{} --- WAITED {}", index, waited_for);
            // Here we send back the value (and consume the sender).
            tx.complete(index);

            println!("{} <-- END", index);
        });
    }

    // `collect` lets us pull through a series of Futures.
    let result = collect(rx_vec)
        // Block until they all are resolved.
        .wait()
        // Note how they all come out in the correct order.
        .map(|values| 
            values.iter()
                .enumerate()
                .all(|(index, &value)| index == value))
        // And finally get the value out.
        .unwrap();

    // Now what was the sum of all the waiting times?
    println!("Job is done. Values returned in order: {}", result);
}
