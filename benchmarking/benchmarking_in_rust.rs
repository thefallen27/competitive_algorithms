use std::time::Instant;

fn main() {
    // Start measuring time
    let start_time = Instant::now();

    // Place your code here that you want to measure

    // End measuring time
    let end_time = Instant::now();

    // Calculate the elapsed time
    let elapsed_time = end_time.duration_since(start_time);

    // Print the elapsed time in milliseconds
    println!("Elapsed time: {} milliseconds", elapsed_time.as_secs_f64() * 1000.0);
}
