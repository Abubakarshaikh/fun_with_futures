extern crate futures;
extern crate fun_with_futures;

use std::thread;
use futures::Future;
use futures::sync::oneshot;

use fun_with_futures::sleep_a_little_bit;

fn main() {
    // This is a simple future built into the crate which feel sort of like
    // one-time channels. You get a (sender, receiver) when you invoke them.
    // Sending a value consumes that side of the channel, leaving only the reciever.
    let (tx, rx) = oneshot::channel();

    // We can spawn a thread to simulate an action that takes time, like a web
    // request. In this case it's just sleeping for a random time.
    thread::spawn(move || {
        println!("--> START");

        let waited_for = sleep_a_little_bit();
        println!("--- WAITED {}", waited_for);
        // This consumes the sender, we can't use it afterwards.
        tx.complete(waited_for);

        println!("<-- END");
    });

    // Now we can wait for it to finish
    let result = rx.wait()
        .unwrap();

    // This value will be the same as the previous "WAITED" output.
    println!("{}", result);
}
