pub fn fib_dynamic(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut fib_vec = vec![0, 1];
    for i in 2..=n {
        let next_fib = fib_vec[(i - 1) as usize] + fib_vec[(i - 2) as usize];
        fib_vec.push(next_fib);
    }
    return fib_vec[n as usize];
}

// fn main() {
//     let result : u64 = fib_dynamic(10);
//     println!("{}", result);
// }
