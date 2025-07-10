mod solution;
use solution::Solution;
fn main() {

    use std::time::Instant;

    let start = Instant::now();

    println!("{}", Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
    println!("{}", Solution::max_area(vec![1,1]));

    println!("Time elapsed : {}ms", start.elapsed().as_micros());
}
