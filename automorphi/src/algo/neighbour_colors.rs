use super::*;

pub struct NeighbourColors;

impl ColorFunc for NeighbourColors {
	fn colorize(&self, g: &Graph, colors: &Vec<usize>) -> Vec<usize> {
		let v: Vec<_> = (0..g.n())
			.map(|x| unorder(g.neighbours(x).map(|y| colors[y]))).collect();

		simplify_colors(v)
	}
}
