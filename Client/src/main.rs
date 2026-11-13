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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_negatives() {
        assert_eq!(is_prime(BigInt::from(-1)), false);
        assert_eq!(is_prime(BigInt::from(-2)), false);
        assert_eq!(is_prime(BigInt::from(-3)), false);
        assert_eq!(is_prime(BigInt::from(-4)), false);
        assert_eq!(is_prime(BigInt::from(-5)), false);
        assert_eq!(is_prime(BigInt::from(-10)), false);
        assert_eq!(is_prime(BigInt::from(-311)), false);
        assert_eq!(is_prime(BigInt::from(-1229)), false);
        assert_eq!(is_prime(BigInt::from(-7919)), false);
    }

    #[test]
    fn test_is_prime_zero() {
        assert_eq!(is_prime(BigInt::from(0)), false);
    }

    #[test]
    fn test_is_prime_positives() {
        assert_eq!(is_prime(BigInt::from(1)), false);
        assert_eq!(is_prime(BigInt::from(4)), false);
        assert_eq!(is_prime(BigInt::from(6)), false);
        assert_eq!(is_prime(BigInt::from(9)), false);
        assert_eq!(is_prime(BigInt::from(423)), false);
        assert_eq!(is_prime(BigInt::from(2649)), false);
        assert_eq!(is_prime(BigInt::from(7909)), false);

        assert_eq!(is_prime(BigInt::from(2)), true);
        assert_eq!(is_prime(BigInt::from(3)), true);
        assert_eq!(is_prime(BigInt::from(5)), true);
        assert_eq!(is_prime(BigInt::from(7)), true);
        assert_eq!(is_prime(BigInt::from(577)), true);
        assert_eq!(is_prime(BigInt::from(1877)), true);
        assert_eq!(is_prime(BigInt::from(2477)), true);
        assert_eq!(is_prime(BigInt::from(7919)), true);
    }
}
