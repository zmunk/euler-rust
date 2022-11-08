mod utils;
mod problem1;
use problem1 as p;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let res = p::main();
    println!("{:?}", res);
    println!("elapsed: {:?}", start.elapsed());
}
