extern crate futures;
extern crate fun_with_futures;
extern crate futures_cpupool;

use futures::future::Future;
use futures_cpupool::Builder;

use fun_with_futures::sleep_a_little_bit;

fn main() {
    // Creates a CpuPool with workers equal to the cores on the machine.
    let pool = Builder::new()
        .create();

    // Note the two spawns return different types.
    let returns_string = pool.spawn_fn(move || {
        sleep_a_little_bit();
        // We need to return a result!
        let result: Result<_, ()> = Ok("First");

        result
    });

    let returns_integer = pool.spawn_fn(move || {
        sleep_a_little_bit();
        // We need to return a result!
        let result: Result<_, ()> = Ok(2);

        result
    });

    // Wait for the jobs to finish.
    let resulting_string = returns_string.wait()
        .unwrap();

    let resulting_integer = returns_integer.wait()
        .unwrap();
    
    println!("{}, {}", resulting_string, resulting_integer);
}
