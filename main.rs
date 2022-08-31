#![allow(unused)]

use cached::cached;
use timer::time_ms;
use std::env;
use std::thread;
use std::time::Duration;

#[cached]
fn fib(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2)
    }
}


fn sleep(seconds: u128) {
    thread::sleep(Duration::from_secs(seconds as u64));
}

fn main() {
    let nums: Vec<u128> = env::args().skip(1).map(|s| s.parse().unwrap()).collect();
    for n in nums {
        time_ms! ({
            let res = fib(n);
            println!("fib({n}):\t{res}");
        });
    }
}
