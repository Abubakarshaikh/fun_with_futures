extern crate futures;
extern crate fun_with_futures;

use std::thread;
use futures::Future;

use fun_with_futures::sleep_a_little_bit;

fn main() {
    // These are simple futures built into the crate which feel sort of like
    // one-time channels. You get a (sender, receiver) when you invoke them.
    let (tx_1, rx_1) = futures::oneshot();
    let (tx_2, rx_2) = futures::oneshot();

    // We can spawn a thread to simulate an action that takes time, like a web
    // request.
    thread::spawn(move || {
        println!("1 --> START");

        let waited_for = sleep_a_little_bit();
        println!("1 --- WAITED {}", waited_for);
        tx_1.complete(waited_for);

        println!("1 <-- END");
    });

    thread::spawn(move || {
        println!("2 --> START");

        let waited_for = sleep_a_little_bit();
        println!("2 --- WAITED {}", waited_for);
        tx_2.complete(waited_for);

        println!("2 <-- END");
    });

    rx_1.join(rx_2)
        .map(|(a, b)| {
            println!("SUM {}", a + b);
        })
        .wait()
        .unwrap();
}
