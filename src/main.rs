use ramp::Int;
use std::time::Instant;
use myrustlib::fib;

fn main() {
    let values: usize = 5000;
    
    let now = Instant::now();
    let big_number: Int = crate::fib(values);
    println!("fibonacci({}): length({}), computed in {}", values, big_number.to_string().len(), now.elapsed().as_millis());
}