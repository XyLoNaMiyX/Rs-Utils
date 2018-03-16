use std::collections::HashSet;
use std::iter::FromIterator;

use super::primeseq;


/// Returns the factors for a given number.
///
/// # Examples
///
/// ```
/// use rs_utils::math::euler::*;
///
/// assert_eq!(factors(231), vec![3, 7, 11]);
/// ```
pub fn factors(mut n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for prime in primeseq::Eratosthenes::new() {
        let mut quotient = n / prime;
        let mut remainder = n % prime;
        while remainder == 0 {
            result.push(prime);
            n = quotient;
            quotient = n / prime;
            remainder = n % prime;
        }

        if n == 1 {
            break;
        }
    }
    result
}

/// Calculates the Euler's Phi function (also known as the Totient function)
/// by using the prime factors of `n`.
///
/// # Examples
///
/// ```
/// use rs_utils::math::euler::*;
///
/// assert_eq!(phi(13), 12);
/// ```
pub fn phi(n: u32) -> u32 {
    let mut result = n as f64;
    for &prime in HashSet::<u32>::from_iter(factors(n)).iter() {
        result *= 1.0 - (1.0 / prime as f64);
    }
    result as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factors() {
        let mut known = Vec::new();
        let mut n = 1;
        for prime in primeseq::Eratosthenes::new() {
            n *= prime;
            known.push(prime);
            assert_eq!(factors(n), known);
            if n > 10_000 {
                break
            }
        }
    }

    #[test]
    fn test_phi() {
        for (n, p) in (1..).zip(vec![1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4]) {
            assert_eq!(phi(n), p);
        }
    }
}
