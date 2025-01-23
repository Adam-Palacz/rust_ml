use std::time::Instant;

mod bubble_sorting;

fn main() {
    let mut v = vec![1.0, 0.0, -2.1, 5.1, -4.4];

    let now = Instant::now();
    let sorted_v = bubble_sorting::bubble_sorting(&mut v);
    let elapsed = now.elapsed();
    println!("bubble_sorting() = {:?}, time: {:?}", sorted_v, elapsed);
}
