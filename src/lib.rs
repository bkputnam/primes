
mod primes_iter;
mod primality_checker;

pub fn iter() -> primes_iter::PrimesIter {
	primes_iter::new()
}

pub fn is_prime(num: u64) -> bool {
	primality_checker::new().is_prime(num)
}

#[test]
fn test_is_prime() {
	assert_eq!(false, is_prime(10));
	assert_eq!(true, is_prime(11));
}