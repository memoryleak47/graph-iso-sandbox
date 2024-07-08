#[cfg(test)]
mod tests;

// self.0[0] is the least significant part.
// each BigInt has at least one vector-element, but other than that as few as possible. This is important for equality checking!
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct BigInt(Vec<u32>);

impl BigInt {
	pub fn new(x: u32) -> BigInt {
		BigInt(vec![x])
	}

	pub fn add(&mut self, other: &BigInt) {
		let mut carry = false;

		let mut i = 0;
		while carry || i < other.0.len() {
			let o: u32 = other.0.get(i).cloned().unwrap_or(0);

			if carry || o > 0 { // if you actually have something to add
				while i >= self.0.len() { self.0.push(0); }
				(self.0[i], carry) = self.0[i].carrying_add(o, carry);
			}

			i += 1;
		}
	}
}
