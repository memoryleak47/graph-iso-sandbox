mod masks;
use masks::MASKS;

#[cfg(test)]
mod tests;

pub struct Boolvec {
	data: Vec<u32>,
}

impl Boolvec {
	pub fn zeros(n: usize) -> Boolvec {
		Boolvec {
			data: vec![0; n/32 + (n%32!=0) as usize],
		}
	}

	pub fn get(&self, i: usize) -> bool {
		self.data[i/32] & MASKS[i%32] > 0
	}

	pub fn set(&mut self, i: usize, val: bool) {
		if self.get(i) != val {
			self.data[i/32] = self.data[i/32] ^ MASKS[i%32];
		}
	}

	#[allow(unused)]
	pub fn to_string(&self, length: usize) -> String {
		const CHR: [char; 2] = ['0', '1'];
		let mut s = String::new();
		for i in 0..length {
			s.push(CHR[self.get(i) as usize]);
		}
		return s;
	}
}
