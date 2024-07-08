use crate::boolvec::Boolvec;

#[cfg(test)]
mod tests;

pub struct Graph {
	matrix: Boolvec,
	n: usize,
}

impl Graph {
	pub fn empty(n: usize) -> Graph {
		Graph {
			matrix: Boolvec::zeros(n*(n-1) / 2),
			n,
		}
	}

	pub fn n(&self) -> usize {
		self.n
	}

	fn idx(&self, i: usize, j: usize) -> usize {
		assert!(i != j);
		let (i, j) = (i.min(j), i.max(j));

		j*(j-1)/2 + i
	}

	pub fn get_edge(&self, i: usize, j: usize) -> bool {
		self.matrix.get(self.idx(i, j))
	}

	pub fn set_edge(&mut self, i: usize, j: usize, val: bool) {
		self.matrix.set(self.idx(i, j), val);
	}

	pub fn neighbours(&self, i: usize) -> impl Iterator<Item=usize> + '_ {
		(0..self.n())
			.filter(move |&j| j != i && self.get_edge(i, j))
	}
}
