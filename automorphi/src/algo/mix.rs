use super::*;

#[derive(Clone, PartialEq)]
struct NumVec(Vec<f32>);

pub struct Mix;

impl ColorFunc for Mix {
	fn colorize(&self, g: &Graph, _colors: &Vec<usize>) -> Vec<usize> {
		let mut v = vec![NumVec::new(g.n()); g.n()];
		for i in 0..g.n() {
			v[i].0[i] = 1f32;
		}
		for _ in 0..g.n() {
			let mut v2 = vec![NumVec::new(g.n()); g.n()];
			for i in 0..g.n() {
				let mut others = vec![&v[i]];
				for j in g.neighbours(i) {
					others.push(&v[j]);
				}
				v2[i].set_to_avg(others);
			}
			v = v2;
		}

		for i in 0..g.n() {
    		v[i].0.sort_by(|a, b| a.partial_cmp(b).unwrap());
		}
		
		simplify_colors(v)
	}
}


// NumVec

impl NumVec {
	fn new(l: usize) -> NumVec {
		NumVec(vec![0f32; l])
	}

	fn set_to_avg(&mut self, others: Vec<&NumVec>) {
		let n = self.0.len();
		for i in 0..n {
			let mut sub_others: Vec<f32> = others.iter().map(|x| x.0[i]).collect();
			sub_others.sort_by(|a, b| a.partial_cmp(b).unwrap()); // this sorting is for stability. sum(sorted(X)) != sum(X) happens sometimes.

			self.0[i] = sub_others.iter().sum::<f32>() / (sub_others.len() as f32);
		}
	}
}
