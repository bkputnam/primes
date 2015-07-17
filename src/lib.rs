
pub struct PrimesIter {
    known_primes: Vec<u64>
}

impl PrimesIter {
	pub fn new() -> PrimesIter {
		PrimesIter {
			known_primes: vec![]
		}
	}

	fn _is_prime(&self, num: u64) -> bool {
		for p in self.known_primes.iter() {
			if num % p == 0 {
				return false
			}
		}
		true
	}

	pub fn is_prime(&self, num: u64) -> Result<bool, &'static str>{
		let last = self.known_primes.last().unwrap();
		if num > last * last {
			return Err("num too large, unable to determine primality")
		}
		Ok(self._is_prime(num))
	}
}

impl Iterator for PrimesIter {
	type Item = u64;

	fn next(&mut self) -> Option<u64> {
		let next = match self.known_primes.last() {
			None => 2,
			Some(last) => {
				let mut current = last+1;
				while !self._is_prime(current) {
					current += 1;
				}
				current
			}
		};
		self.known_primes.push(next);
		Some(next)
	}
}

pub fn iter() -> PrimesIter {
	PrimesIter::new()
}

#[test]
fn it_works() {
	let mut pi = iter();
	
	assert_eq!(Some(2), pi.next());
	assert_eq!(Some(3), pi.next());
	assert_eq!(Some(5), pi.next());
	assert_eq!(Some(7), pi.next());
	assert_eq!(Some(11), pi.next());
	assert_eq!(Some(13), pi.next());
	assert_eq!(Some(17), pi.next());
	assert_eq!(Some(19), pi.next());
	assert_eq!(Some(23), pi.next());
}
