extern crate futures;
extern crate fun_with_futures;
extern crate futures_cpupool;

use futures::future::{Future, join_all, lazy, ok};
use futures_cpupool::CpuPool;

use fun_with_futures::sleep_a_little_bit;

fn main() {
    let pool = CpuPool::new(4);

    // We can spawn a thread to simulate an action that takes time, like a web
    // request.
    let futures = (1..100).map(|index|
        lazy(move || {
            println!("{} --> START", index);
            let waited_for = sleep_a_little_bit();
            println!("{} <-- WAITED {}", index, waited_for);
            ok::<u64, String>(waited_for)
        })
      )
      .map(|future| pool.spawn(future))
      .collect::<Vec<_>>();

    let sum = join_all(futures)
      .map(|results| results.iter().fold(0, |sum, val| sum + val))
      .wait()
      .unwrap();
    println!("SUM: {:?}", sum)
}
