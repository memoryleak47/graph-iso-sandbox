use crate::Graph;

use std::collections::HashMap;
use std::hash::Hash;

mod neighbour_colors; use neighbour_colors::NeighbourColors;
mod mix;
mod bigintmix; use bigintmix::BigIntMix;

#[cfg(test)]
mod tests;

pub trait ColorFunc {
	fn colorize(&self, g: &Graph, colors: &Vec<usize>) -> Vec<usize>;
}

// this could be faster by sorting instead of this n^2 algorithm, but then we'd require T to be Ord
pub(self) fn simplify_colors<T: PartialEq>(v: Vec<T>) -> Vec<usize> {
	let mut out = vec![v.len(); v.len()];
	let mut calced = vec![false; v.len()];
	let mut next_color = 0;

	for i in 0..v.len() {
		if calced[i] { continue; }

		out[i] = next_color;
		calced[i] = true;

		for j in (i+1)..v.len() {
			if calced[j] { continue; }

			if v[i] == v[j] {
				calced[j] = true;
				out[j] = next_color;
			}
		}

		next_color += 1;
	}

	out
}

pub(self) fn unorder<T: Eq + Hash>(it: impl Iterator<Item=T>) -> HashMap<T, usize> {
	let mut map = HashMap::new();
	for x in it {
		*map.entry(x).or_insert(0) += 1;
	}

	map
}

fn count_colors(v: &Vec<usize>) -> usize {
	let mut v = v.clone();
	v.sort();
	v.dedup();

	v.len()
}

struct FullColorFunc;

impl ColorFunc for FullColorFunc {
	fn colorize(&self, g: &Graph, colors: &Vec<usize>) -> Vec<usize> {
		let a = NeighbourColors.colorize(g, colors);
		let b = BigIntMix.colorize(g, colors);
		let v = (0..g.n()).map(|i| (colors[i], a[i], b[i])).collect();

		simplify_colors(v)
	}
}

impl Graph {
	// returns whether the algorithm found a proof for asymmetry.
	// this algorithm is poly-time, but will not find all asymmetries.
	pub fn search_asymmetry_by(&self, f: impl ColorFunc) -> bool {
		let mut colors = vec![0; self.n()];
		let mut count = 1; // initially there is just one color, namely 0.
		
		loop {
			colors = f.colorize(self, &colors);

			let new_count = count_colors(&colors);
			if new_count == count { break; }
			else { count = new_count; }
		}

		count == self.n()
	}

	pub fn search_asymmetry(&self) -> bool {
		self.search_asymmetry_by(FullColorFunc)
	}
}
