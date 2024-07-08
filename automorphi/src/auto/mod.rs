use std::ops::Index;
use crate::graph::Graph;

pub struct Permutation(Vec<usize>);

impl Index<usize> for Permutation {
	type Output = usize;

	fn index(&self, i: usize) -> &usize {
		&self.0[i]
	}
}

impl Graph {
	pub fn is_automorphism(&self, p: &Permutation) -> bool {
		for i in 0..self.n() {
			for j in 0..self.n() {
				if self.get_edge(i, j) != self.get_edge(p[i], p[j]) {
					return false;
				}
			}
		}

		true
	}
}
