use super::*;
use crate::bigint::BigInt;

#[derive(Clone, PartialEq)]
struct NumVec(Vec<BigInt>);

pub struct BigIntMix;

impl ColorFunc for BigIntMix {
	fn colorize(&self, g: &Graph, _colors: &Vec<usize>) -> Vec<usize> {
		let mut v = vec![NumVec::new(g.n()); g.n()];
		for i in 0..g.n() {
			v[i].0[i] = BigInt::new(1);
		}
		for _ in 0..g.n() {
			let mut v2 = vec![NumVec::new(g.n()); g.n()];
			for i in 0..g.n() {
				let mut others = vec![&v[i]];
				for j in g.neighbours(i) {
					others.push(&v[j]);
				}
				v2[i].add_others(others);
			}
			v = v2;
		}

		let mut out = Vec::new();
		for i in 0..g.n() {
			out.push(unorder(v[i].0.iter()));
		}
		
		simplify_colors(out)
	}
}


// NumVec

impl NumVec {
	fn new(l: usize) -> NumVec {
		NumVec(vec![BigInt::new(0); l])
	}

	fn add_others(&mut self, others: Vec<&NumVec>) {
		let n = self.0.len();
		for i in 0..n {
			for o in &others {
				self.0[i].add(&o.0[i]);
			}
		}
	}
}
