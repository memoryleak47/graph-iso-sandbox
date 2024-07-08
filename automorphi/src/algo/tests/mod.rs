use crate::Graph;

mod igraph;

#[test]
fn empty_is_symmetric() {
	assert_eq!(Graph::empty(4).search_asymmetry(), false);
}

#[test]
fn this_claw_is_asymmetric() {
	let mut g = Graph::empty(7);

	g.set_edge(0, 1, true);

	g.set_edge(0, 2, true);
	g.set_edge(2, 3, true);

	g.set_edge(0, 4, true);
	g.set_edge(4, 5, true);
	g.set_edge(5, 6, true);

	assert_eq!(g.search_asymmetry(), true);
}
