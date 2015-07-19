
use primes_iter;

pub fn new() -> PrimalityChecker {
	PrimalityChecker::new()
}

pub struct PrimalityChecker {
	pi: primes_iter::PrimesIter
}

impl PrimalityChecker {

	pub fn new() -> PrimalityChecker {
		PrimalityChecker {
			pi: primes_iter::new()
		}
	}

	pub fn is_prime(&mut self, num: u64) -> bool {

		// Short-circuit for cases where num is
		// small enough that we can just search
		// for it in known_primes
		let last = match self.pi.known_primes.last() {
			None => 0,
			Some(last) => *last
		};
		if num <= last {
			for p in self.pi.known_primes.iter().take_while(|&p| *p <= num) {
				if *p == num {
					return true
				}
			}
			return false
		};

		let sqrt = (num as f64).sqrt().ceil() as u64;

		while
			self.pi.known_primes.is_empty() ||
			*self.pi.known_primes.last().unwrap() < sqrt
		{
			self.pi.next();
		}

		match self.pi.is_prime(num) {
			Err(_) => panic!("Programming error: this should be impossible"),
			Ok(result) => result
		}
	}
}

#[test]
fn test_is_prime() {

	let mut pc = new();

	assert_eq!(true, pc.is_prime(2));
	assert_eq!(true, pc.is_prime(3));
	assert_eq!(false, pc.is_prime(4));
	assert_eq!(true, pc.is_prime(5));
	assert_eq!(false, pc.is_prime(6));
	assert_eq!(true, pc.is_prime(7));
	assert_eq!(false, pc.is_prime(8));
	assert_eq!(false, pc.is_prime(9));
	assert_eq!(false, pc.is_prime(10));
	assert_eq!(true, pc.is_prime(11));

	// Second iteration should check the
	// cases where num < known_primes.last()
	assert_eq!(true, pc.is_prime(2));
	assert_eq!(true, pc.is_prime(3));
	assert_eq!(false, pc.is_prime(4));
	assert_eq!(true, pc.is_prime(5));
	assert_eq!(false, pc.is_prime(6));
	assert_eq!(true, pc.is_prime(7));
	assert_eq!(false, pc.is_prime(8));
	assert_eq!(false, pc.is_prime(9));
	assert_eq!(false, pc.is_prime(10));
	assert_eq!(true, pc.is_prime(11));
}

