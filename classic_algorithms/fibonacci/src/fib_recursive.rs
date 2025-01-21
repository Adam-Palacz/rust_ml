pub fn fib_recursive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    } else {
        return fib_recursive(n - 1) + fib_recursive(n - 2);
    }
}

// fn main() {
//     let result : u64 = fib_recursive(10);
//     println!("{}", result);
// }
