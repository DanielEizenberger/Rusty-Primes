use num_bigint::BigInt;
use serde_json::json;
use std::thread;

const BASE_URL: &str = "http://localhost:5185";

#[tokio::main]
async fn main() {
    calculate_primes(1.0).await;
}

async fn get_number() -> Result<BigInt, Box<dyn std::error::Error>> {
    let url = format!("{}/job", BASE_URL);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let number = body.parse::<BigInt>()?;
    Ok(number)
}

async fn post_is_prime(num: &BigInt, is_prime: bool) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}/job", BASE_URL);
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .json(&json!({
            "number": num.to_string(),
            "isPrime": is_prime
        }))
        .send()
        .await?;

    let body = response.text().await?;

    Ok(body)
}

async fn calc_prime_remote() {
    let response = get_number().await;
    let num = response.unwrap();
    let is_prime = is_prime(&num);
    if is_prime {
        println!("{} is a prime number!", num);
    }

    post_is_prime(&num, is_prime).await;
}

async fn calculate_primes(cpu_usage: f32) {
    if cpu_usage < 0.0 || cpu_usage > 1.0 {
        print!("CPU usage can only be between 0.0 and 1.0");
        return;
    }

    let thread_count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1) as u64;
    let threads = ((thread_count as f32) * cpu_usage).max(1f32) as u64;

    println!("Available CPU threads: {}", thread_count);
    println!("Used CPU threads: {}", threads);

    let mut handles = vec![];

    for _ in 0..threads {
        let handle = thread::spawn(async move || {
            loop {
                calc_prime_remote().await;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap().await;
    }
}

fn is_prime(n: &BigInt) -> bool {
    if n <= &BigInt::ONE {
        return false;
    } else if n == &BigInt::from(2) {
        return true;
    } else if n % &BigInt::from(2) == BigInt::ZERO {
        return false;
    }

    let mut counter = BigInt::from(3);
    let top = n.sqrt();
    while counter <= top {
        if n % &counter == BigInt::ZERO {
            return false;
        }
        counter += 1;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_negatives() {
        assert_eq!(is_prime(&BigInt::from(-1)), false);
        assert_eq!(is_prime(&BigInt::from(-2)), false);
        assert_eq!(is_prime(&BigInt::from(-3)), false);
        assert_eq!(is_prime(&BigInt::from(-4)), false);
        assert_eq!(is_prime(&BigInt::from(-5)), false);
        assert_eq!(is_prime(&BigInt::from(-10)), false);
        assert_eq!(is_prime(&BigInt::from(-311)), false);
        assert_eq!(is_prime(&BigInt::from(-1229)), false);
        assert_eq!(is_prime(&BigInt::from(-7919)), false);
    }

    #[test]
    fn test_is_prime_zero() {
        assert_eq!(is_prime(&BigInt::from(0)), false);
    }

    #[test]
    fn test_is_prime_positives() {
        assert_eq!(is_prime(&BigInt::from(1)), false);
        assert_eq!(is_prime(&BigInt::from(4)), false);
        assert_eq!(is_prime(&BigInt::from(6)), false);
        assert_eq!(is_prime(&BigInt::from(9)), false);
        assert_eq!(is_prime(&BigInt::from(423)), false);
        assert_eq!(is_prime(&BigInt::from(2649)), false);
        assert_eq!(is_prime(&BigInt::from(7909)), false);

        assert_eq!(is_prime(&BigInt::from(2)), true);
        assert_eq!(is_prime(&BigInt::from(3)), true);
        assert_eq!(is_prime(&BigInt::from(5)), true);
        assert_eq!(is_prime(&BigInt::from(7)), true);
        assert_eq!(is_prime(&BigInt::from(577)), true);
        assert_eq!(is_prime(&BigInt::from(1877)), true);
        assert_eq!(is_prime(&BigInt::from(2477)), true);
        assert_eq!(is_prime(&BigInt::from(7919)), true);
    }
}
