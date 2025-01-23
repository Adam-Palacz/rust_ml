use std::time::Instant;

mod fib_recursive;
mod fib_iterative;
mod fib_dynamic;

fn main() {
    let n = 30;

    let now = Instant::now();
    let fib_recursive_result = fib_recursive::fib_recursive(n);
    let elapsed = now.elapsed();
    println!("fib_recursive({}) = {}, time: {:?}", n, fib_recursive_result, elapsed);

    let now = Instant::now();
    let fib_iterative_result = fib_iterative::fib_iterative(n);
    let elapsed = now.elapsed();
    println!("fib_iterative({}) = {}, time: {:?}", n, fib_iterative_result, elapsed);

    let now = Instant::now();
    let fib_dynamic_result = fib_dynamic::fib_dynamic(n);
    let elapsed = now.elapsed();
    println!("fib_dynamic({}) = {}, time: {:?}", n, fib_dynamic_result, elapsed);
}