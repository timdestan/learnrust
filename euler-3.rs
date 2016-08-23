
// Builds a vector mapping index to is_prime(index + 1).
fn sieve(size: usize) -> Vec<bool> {
  let mut is_primes:Vec<bool> = vec![true; size];
  is_primes[0] = false;  // 1
  let mut i: usize = 1;
  while i < size {
    if is_primes[i] {
      let factor = i + 1;
      // Mark indexes of all multiples as composite.
      let mut j = i + factor;
      while j < size {
        is_primes[j] = false;
        j += factor;
      }
    }
    i += 1;
  }
  is_primes
}

fn prime_factors(n: usize) -> Vec<usize> {
  let mut factors: Vec<usize> = vec![];
  let sieve: Vec<bool> = sieve((n as f64).sqrt() as usize + 2);
  let mut remainder = n;
  while remainder >= 1 &&
        (remainder - 1 >= sieve.len() ||
         !sieve[remainder - 1]) {
    let mut matched = false;
    let mut i:usize = 0;
    for prime in &sieve {
      if i > 0 && *prime && remainder % (i + 1) == 0 {
        matched = true;
        remainder = remainder / (i + 1);
        factors.push(i + 1);
        break;
      }
      i += 1;
    }
    if !matched {
      break;
    }
  }
  factors.push(remainder);
  factors
}

fn largest_prime_factor(n: usize) -> usize {
  match prime_factors(n).last() {
    None => 0,
    Some(factor) => *factor,
  }
}

fn main() {
  let n:usize = 600851475143;
  println!("largest prime factor of {} is {}", n, largest_prime_factor(n));
}
