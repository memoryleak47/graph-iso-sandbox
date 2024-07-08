use crate::graph::Graph;

#[test]
fn test() {
	const N: usize = 10;
	for i in 0..N {
		for j in 0..N {
			if i == j { continue; }

			let mut g = Graph::empty(N);
			g.set_edge(i, j, true);

			for i2 in 0..N {
				for j2 in 0..N {
					if i2 == j2 { continue; }

					let should = i.min(j) == i2.min(j2) && i.max(j) == i2.max(j2);
					assert_eq!(g.get_edge(i2, j2), should);
				}
			}
		}
	}
}
