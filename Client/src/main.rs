use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::thread;

fn main() {
    let limit: u64 = 1_000_000;
    let thread_count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1) as u64;

    println!("Available CPU threads: {}", thread_count);

    let primes = Arc::new(Mutex::new(Vec::new()));
    primes.lock().unwrap().push(2);
    let mut handles = vec![];

    let start = Instant::now();
    let chunk_size = limit / thread_count;
    for chunk in 0..thread_count {
        let primes = Arc::clone(&primes);
        let handle = thread::spawn(move || {
            let start = chunk * chunk_size + 1;
            let end = (chunk + 1) * chunk_size;
            for i in start..=end {
                if i % 2 == 0 {
                    continue;
                }

                if is_prime(i) {
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

fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    else if n == 2 { return true; }
    else if n % 2 == 0 { return false; }

    for i in 3..=((n as f64).sqrt() as u64) {
        if n % i == 0 { return false; }
    }

    return true;
}
