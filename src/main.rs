mod solution;
use solution::Solution;

fn main() {

    use std::time::Instant;

    let start = Instant::now();

    println!("{}", Solution::int_to_roman(3479));
    println!("{}", Solution::int_to_roman(58));
    println!("{}", Solution::int_to_roman(1194));

    println!("Time elapsed : {}ms", start.elapsed().as_micros());
}
