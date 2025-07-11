mod solution;
use solution::Solution;

fn main() {

    use std::time::Instant;

    let start = Instant::now();

    println!("Time elapsed : {}ms", start.elapsed().as_micros());
}
