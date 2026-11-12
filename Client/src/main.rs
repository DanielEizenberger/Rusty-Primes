use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::thread;
use num_bigint::BigInt;

fn main() {
    calculate_primes(1.0);
}

fn calculate_primes(cpu_usage: f32) {
    if cpu_usage < 0.0 || cpu_usage > 1.0 {
        print!("CPU usage can only be between 0.0 and 1.0");
        return;
    }

    let limit: u64 = 1_000_000;
    let thread_count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1) as u64;
    let threads = ((thread_count as f32) * cpu_usage).max(1f32) as u64;

    println!("Available CPU threads: {}", thread_count);

    let primes = Arc::new(Mutex::new(Vec::new()));
    primes.lock().unwrap().push(2);
    let mut handles = vec![];

    let start = Instant::now();
    let chunk_size = limit / threads;
    for chunk in 0..threads {
        let primes = Arc::clone(&primes);
        let handle = thread::spawn(move || {
            let start = chunk * chunk_size + 1;
            let end = (chunk + 1) * chunk_size;
            for i in start..=end {
                if i % 2 == 0 {
                    continue;
                }

                if is_prime(BigInt::from(i)) {
                    primes.lock().unwrap().push(i);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);

    println!("Sorting primes..."); // because, why not
    primes.lock().unwrap().sort_unstable();
    println!("All primes sorted!");

    println!("Primes: {}", primes.lock().unwrap().len());
}

fn is_prime(n: BigInt) -> bool {
    if n <= BigInt::ONE { return false; }
    else if n == BigInt::from(2) { return true; }
    else if &n % BigInt::from(2) == BigInt::ZERO { return false; }

    let mut counter = BigInt::from(3);
    let top = n.sqrt();
    while counter <= top {
        if &n % &counter == BigInt::ZERO { return false; }
        counter += 1;
    }

    return true;
}
