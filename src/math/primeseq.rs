use std::collections::HashMap;


/// Determines if the given number is prime.
///
/// # Examples
///
/// ```
/// use rs_utils::math::primeseq::*;
///
/// assert!(is_prime(7));
/// assert!(!is_prime(8));
/// ```
pub fn is_prime(n: u32) -> bool {
    match n {
        n if n < 4 => n >= 2,
        n if n % 2 == 0 => false,
        n if ((n - 5) % 6 != 0) && ((n - 7) % 6 != 0) => false,
        n => {
            let mut i = 3;
            let top = (n as f64).sqrt() as u32 + 1;
            while i < top {
                if n % i == 0 {
                    return false;
                } else {
                    i += 2;
                }
            }
            true
        }
    }
}

/// Structure to hold the data needed for running the Eratosthenes sieve.
/// Its primary use is being an iterator, yielding primes *ad infinitum*.
pub struct Eratosthenes {
    q: u32,
    d: HashMap<u32, u32>
}

impl Eratosthenes {
    pub fn new() -> Eratosthenes {
        Eratosthenes { q: 2, d: HashMap::new() }
    }
}

impl Iterator for Eratosthenes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.q == 2 {
            self.q = 3;
            return Some(2)
        }
        loop {
            match self.d.remove(&self.q) {
                Some(p) => {
                    let p2 = p * 2;
                    let mut x = p2 + self.q;
                    while self.d.contains_key(&x) {
                        x += p2;
                    }
                    self.d.insert(x, p);
                    self.q += 2;
                },
                None => {
                    self.d.insert(self.q * self.q, self.q);
                    self.q += 2;
                    return Some(self.q - 2)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_primes() {
        for prime in vec![2, 3, 5, 7, 11] {
            assert!(is_prime(prime));
        }

        for not_prime in vec![0, 1, 4, 6, 8] {
            assert!(!is_prime(not_prime));
        }
    }

    #[test]
    fn test_eratosthenes() {
        for prime in Eratosthenes::new() {
            if prime < 10_000 {
                assert!(is_prime(prime));
            } else {
                break
            }
        }
    }
}
