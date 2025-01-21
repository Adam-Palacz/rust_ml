pub fn fib_iterative(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    return b;
}

// fn main() {
//     let result : u64 = fib_iterative(10);
//     println!("{}", result);
// }
