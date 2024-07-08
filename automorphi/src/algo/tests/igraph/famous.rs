use crate::Graph;

#[test]
fn igraph_bull_is_symmetric() {
	let mut g = Graph::empty(5);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 3, true);
	g.set_edge(2, 4, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_chvatal_is_symmetric() {
	let mut g = Graph::empty(12);

	g.set_edge(0, 6, true);
	g.set_edge(0, 9, true);
	g.set_edge(0, 10, true);
	g.set_edge(0, 11, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 5, true);
	g.set_edge(1, 7, true);
	g.set_edge(1, 10, true);
	g.set_edge(2, 6, true);
	g.set_edge(2, 8, true);
	g.set_edge(2, 11, true);
	g.set_edge(3, 7, true);
	g.set_edge(3, 9, true);
	g.set_edge(3, 10, true);
	g.set_edge(3, 11, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 8, true);
	g.set_edge(4, 10, true);
	g.set_edge(4, 11, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 9, true);
	g.set_edge(6, 7, true);
	g.set_edge(7, 8, true);
	g.set_edge(8, 9, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_coxeter_is_symmetric() {
	let mut g = Graph::empty(28);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 7, true);
	g.set_edge(1, 4, true);
	g.set_edge(1, 13, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 8, true);
	g.set_edge(3, 6, true);
	g.set_edge(3, 9, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 12, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 11, true);
	g.set_edge(6, 10, true);
	g.set_edge(7, 19, true);
	g.set_edge(7, 24, true);
	g.set_edge(8, 20, true);
	g.set_edge(8, 23, true);
	g.set_edge(9, 14, true);
	g.set_edge(9, 22, true);
	g.set_edge(10, 15, true);
	g.set_edge(10, 21, true);
	g.set_edge(11, 16, true);
	g.set_edge(11, 27, true);
	g.set_edge(12, 17, true);
	g.set_edge(12, 26, true);
	g.set_edge(13, 18, true);
	g.set_edge(13, 25, true);
	g.set_edge(14, 17, true);
	g.set_edge(14, 18, true);
	g.set_edge(15, 18, true);
	g.set_edge(15, 19, true);
	g.set_edge(16, 19, true);
	g.set_edge(16, 20, true);
	g.set_edge(17, 20, true);
	g.set_edge(21, 23, true);
	g.set_edge(21, 26, true);
	g.set_edge(22, 24, true);
	g.set_edge(22, 27, true);
	g.set_edge(23, 25, true);
	g.set_edge(24, 26, true);
	g.set_edge(25, 27, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_cubical_is_symmetric() {
	let mut g = Graph::empty(8);

	g.set_edge(0, 1, true);
	g.set_edge(0, 3, true);
	g.set_edge(0, 4, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 5, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 6, true);
	g.set_edge(3, 7, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 7, true);
	g.set_edge(5, 6, true);
	g.set_edge(6, 7, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_diamond_is_symmetric() {
	let mut g = Graph::empty(4);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 3, true);
	g.set_edge(2, 3, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_dodecahedral_is_symmetric() {
	let mut g = Graph::empty(20);

	g.set_edge(0, 1, true);
	g.set_edge(0, 4, true);
	g.set_edge(0, 5, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 6, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 7, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 8, true);
	g.set_edge(4, 9, true);
	g.set_edge(5, 10, true);
	g.set_edge(5, 11, true);
	g.set_edge(6, 10, true);
	g.set_edge(6, 14, true);
	g.set_edge(7, 13, true);
	g.set_edge(7, 14, true);
	g.set_edge(8, 12, true);
	g.set_edge(8, 13, true);
	g.set_edge(9, 11, true);
	g.set_edge(9, 12, true);
	g.set_edge(10, 15, true);
	g.set_edge(11, 16, true);
	g.set_edge(12, 17, true);
	g.set_edge(13, 18, true);
	g.set_edge(14, 19, true);
	g.set_edge(15, 16, true);
	g.set_edge(15, 19, true);
	g.set_edge(16, 17, true);
	g.set_edge(17, 18, true);
	g.set_edge(18, 19, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_folkman_is_symmetric() {
	let mut g = Graph::empty(20);

	g.set_edge(0, 5, true);
	g.set_edge(0, 8, true);
	g.set_edge(0, 10, true);
	g.set_edge(0, 13, true);
	g.set_edge(1, 7, true);
	g.set_edge(1, 9, true);
	g.set_edge(1, 12, true);
	g.set_edge(1, 14, true);
	g.set_edge(2, 6, true);
	g.set_edge(2, 8, true);
	g.set_edge(2, 11, true);
	g.set_edge(2, 13, true);
	g.set_edge(3, 5, true);
	g.set_edge(3, 7, true);
	g.set_edge(3, 10, true);
	g.set_edge(3, 12, true);
	g.set_edge(4, 6, true);
	g.set_edge(4, 9, true);
	g.set_edge(4, 11, true);
	g.set_edge(4, 14, true);
	g.set_edge(5, 15, true);
	g.set_edge(5, 19, true);
	g.set_edge(6, 15, true);
	g.set_edge(6, 16, true);
	g.set_edge(7, 16, true);
	g.set_edge(7, 17, true);
	g.set_edge(8, 17, true);
	g.set_edge(8, 18, true);
	g.set_edge(9, 18, true);
	g.set_edge(9, 19, true);
	g.set_edge(10, 15, true);
	g.set_edge(10, 19, true);
	g.set_edge(11, 15, true);
	g.set_edge(11, 16, true);
	g.set_edge(12, 16, true);
	g.set_edge(12, 17, true);
	g.set_edge(13, 17, true);
	g.set_edge(13, 18, true);
	g.set_edge(14, 18, true);
	g.set_edge(14, 19, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_franklin_is_symmetric() {
	let mut g = Graph::empty(12);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 6, true);
	g.set_edge(1, 3, true);
	g.set_edge(1, 7, true);
	g.set_edge(2, 4, true);
	g.set_edge(2, 10, true);
	g.set_edge(3, 5, true);
	g.set_edge(3, 11, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 6, true);
	g.set_edge(5, 7, true);
	g.set_edge(6, 8, true);
	g.set_edge(7, 9, true);
	g.set_edge(8, 9, true);
	g.set_edge(8, 11, true);
	g.set_edge(9, 10, true);
	g.set_edge(10, 11, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_frucht_is_asymmetric() {
	let mut g = Graph::empty(12);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 11, true);
	g.set_edge(1, 3, true);
	g.set_edge(1, 6, true);
	g.set_edge(2, 5, true);
	g.set_edge(2, 10, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 6, true);
	g.set_edge(4, 8, true);
	g.set_edge(4, 11, true);
	g.set_edge(5, 9, true);
	g.set_edge(5, 10, true);
	g.set_edge(6, 7, true);
	g.set_edge(7, 8, true);
	g.set_edge(7, 9, true);
	g.set_edge(8, 9, true);
	g.set_edge(10, 11, true);

	assert_eq!(g.search_asymmetry(), true);
}
#[test]
fn igraph_grotzsch_is_symmetric() {
	let mut g = Graph::empty(11);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 7, true);
	g.set_edge(0, 10, true);
	g.set_edge(1, 3, true);
	g.set_edge(1, 6, true);
	g.set_edge(1, 9, true);
	g.set_edge(2, 4, true);
	g.set_edge(2, 6, true);
	g.set_edge(2, 8, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 8, true);
	g.set_edge(3, 10, true);
	g.set_edge(4, 7, true);
	g.set_edge(4, 9, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 7, true);
	g.set_edge(5, 8, true);
	g.set_edge(5, 9, true);
	g.set_edge(5, 10, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_heawood_is_symmetric() {
	let mut g = Graph::empty(14);

	g.set_edge(0, 1, true);
	g.set_edge(0, 5, true);
	g.set_edge(0, 13, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 10, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 7, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 12, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 9, true);
	g.set_edge(5, 6, true);
	g.set_edge(6, 7, true);
	g.set_edge(6, 11, true);
	g.set_edge(7, 8, true);
	g.set_edge(8, 9, true);
	g.set_edge(8, 13, true);
	g.set_edge(9, 10, true);
	g.set_edge(10, 11, true);
	g.set_edge(11, 12, true);
	g.set_edge(12, 13, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_herschel_is_symmetric() {
	let mut g = Graph::empty(11);

	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(0, 4, true);
	g.set_edge(0, 5, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 3, true);
	g.set_edge(1, 6, true);
	g.set_edge(1, 7, true);
	g.set_edge(2, 10, true);
	g.set_edge(3, 9, true);
	g.set_edge(4, 8, true);
	g.set_edge(4, 9, true);
	g.set_edge(5, 8, true);
	g.set_edge(5, 10, true);
	g.set_edge(6, 8, true);
	g.set_edge(6, 9, true);
	g.set_edge(7, 8, true);
	g.set_edge(7, 10, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_house_is_symmetric() {
	let mut g = Graph::empty(5);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(1, 3, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 4, true);
	g.set_edge(3, 4, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_housex_is_symmetric() {
	let mut g = Graph::empty(5);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 3, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 4, true);
	g.set_edge(3, 4, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_icosahedral_is_symmetric() {
	let mut g = Graph::empty(12);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(0, 4, true);
	g.set_edge(0, 8, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 6, true);
	g.set_edge(1, 7, true);
	g.set_edge(1, 8, true);
	g.set_edge(2, 4, true);
	g.set_edge(2, 5, true);
	g.set_edge(2, 6, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 8, true);
	g.set_edge(3, 9, true);
	g.set_edge(3, 11, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 11, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 10, true);
	g.set_edge(5, 11, true);
	g.set_edge(6, 7, true);
	g.set_edge(6, 10, true);
	g.set_edge(7, 8, true);
	g.set_edge(7, 9, true);
	g.set_edge(7, 10, true);
	g.set_edge(8, 9, true);
	g.set_edge(9, 10, true);
	g.set_edge(9, 11, true);
	g.set_edge(10, 11, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_krackhardt_kite_is_symmetric() {
	let mut g = Graph::empty(10);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(0, 5, true);
	g.set_edge(1, 3, true);
	g.set_edge(1, 4, true);
	g.set_edge(1, 6, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 5, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 5, true);
	g.set_edge(3, 6, true);
	g.set_edge(4, 6, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 7, true);
	g.set_edge(6, 7, true);
	g.set_edge(7, 8, true);
	g.set_edge(8, 9, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_levi_is_symmetric() {
	let mut g = Graph::empty(30);

	g.set_edge(0, 1, true);
	g.set_edge(0, 7, true);
	g.set_edge(0, 29, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 24, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 11, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 16, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 21, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 26, true);
	g.set_edge(6, 7, true);
	g.set_edge(6, 13, true);
	g.set_edge(7, 8, true);
	g.set_edge(8, 9, true);
	g.set_edge(8, 17, true);
	g.set_edge(9, 10, true);
	g.set_edge(9, 22, true);
	g.set_edge(10, 11, true);
	g.set_edge(10, 27, true);
	g.set_edge(11, 12, true);
	g.set_edge(12, 13, true);
	g.set_edge(12, 19, true);
	g.set_edge(13, 14, true);
	g.set_edge(14, 15, true);
	g.set_edge(14, 23, true);
	g.set_edge(15, 16, true);
	g.set_edge(15, 28, true);
	g.set_edge(16, 17, true);
	g.set_edge(17, 18, true);
	g.set_edge(18, 19, true);
	g.set_edge(18, 25, true);
	g.set_edge(19, 20, true);
	g.set_edge(20, 21, true);
	g.set_edge(20, 29, true);
	g.set_edge(21, 22, true);
	g.set_edge(22, 23, true);
	g.set_edge(23, 24, true);
	g.set_edge(24, 25, true);
	g.set_edge(25, 26, true);
	g.set_edge(26, 27, true);
	g.set_edge(27, 28, true);
	g.set_edge(28, 29, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_mcgee_is_symmetric() {
	let mut g = Graph::empty(24);

	g.set_edge(0, 1, true);
	g.set_edge(0, 7, true);
	g.set_edge(0, 23, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 18, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 14, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 10, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 21, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 17, true);
	g.set_edge(6, 7, true);
	g.set_edge(6, 13, true);
	g.set_edge(7, 8, true);
	g.set_edge(8, 9, true);
	g.set_edge(8, 20, true);
	g.set_edge(9, 10, true);
	g.set_edge(9, 16, true);
	g.set_edge(10, 11, true);
	g.set_edge(11, 12, true);
	g.set_edge(11, 23, true);
	g.set_edge(12, 13, true);
	g.set_edge(12, 19, true);
	g.set_edge(13, 14, true);
	g.set_edge(14, 15, true);
	g.set_edge(15, 16, true);
	g.set_edge(15, 22, true);
	g.set_edge(16, 17, true);
	g.set_edge(17, 18, true);
	g.set_edge(18, 19, true);
	g.set_edge(19, 20, true);
	g.set_edge(20, 21, true);
	g.set_edge(21, 22, true);
	g.set_edge(22, 23, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_meredith_is_symmetric() {
	let mut g = Graph::empty(70);

	g.set_edge(0, 4, true);
	g.set_edge(0, 5, true);
	g.set_edge(0, 6, true);
	g.set_edge(0, 17, true);
	g.set_edge(1, 4, true);
	g.set_edge(1, 5, true);
	g.set_edge(1, 6, true);
	g.set_edge(1, 51, true);
	g.set_edge(2, 4, true);
	g.set_edge(2, 5, true);
	g.set_edge(2, 6, true);
	g.set_edge(2, 50, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 5, true);
	g.set_edge(3, 6, true);
	g.set_edge(3, 21, true);
	g.set_edge(7, 11, true);
	g.set_edge(7, 12, true);
	g.set_edge(7, 13, true);
	g.set_edge(7, 24, true);
	g.set_edge(8, 11, true);
	g.set_edge(8, 12, true);
	g.set_edge(8, 13, true);
	g.set_edge(8, 58, true);
	g.set_edge(9, 11, true);
	g.set_edge(9, 12, true);
	g.set_edge(9, 13, true);
	g.set_edge(9, 57, true);
	g.set_edge(10, 11, true);
	g.set_edge(10, 12, true);
	g.set_edge(10, 13, true);
	g.set_edge(10, 28, true);
	g.set_edge(14, 18, true);
	g.set_edge(14, 19, true);
	g.set_edge(14, 20, true);
	g.set_edge(14, 31, true);
	g.set_edge(15, 18, true);
	g.set_edge(15, 19, true);
	g.set_edge(15, 20, true);
	g.set_edge(15, 65, true);
	g.set_edge(16, 18, true);
	g.set_edge(16, 19, true);
	g.set_edge(16, 20, true);
	g.set_edge(16, 64, true);
	g.set_edge(17, 18, true);
	g.set_edge(17, 19, true);
	g.set_edge(17, 20, true);
	g.set_edge(21, 25, true);
	g.set_edge(21, 26, true);
	g.set_edge(21, 27, true);
	g.set_edge(22, 25, true);
	g.set_edge(22, 26, true);
	g.set_edge(22, 27, true);
	g.set_edge(22, 37, true);
	g.set_edge(23, 25, true);
	g.set_edge(23, 26, true);
	g.set_edge(23, 27, true);
	g.set_edge(23, 36, true);
	g.set_edge(24, 25, true);
	g.set_edge(24, 26, true);
	g.set_edge(24, 27, true);
	g.set_edge(28, 32, true);
	g.set_edge(28, 33, true);
	g.set_edge(28, 34, true);
	g.set_edge(29, 32, true);
	g.set_edge(29, 33, true);
	g.set_edge(29, 34, true);
	g.set_edge(29, 44, true);
	g.set_edge(30, 32, true);
	g.set_edge(30, 33, true);
	g.set_edge(30, 34, true);
	g.set_edge(30, 43, true);
	g.set_edge(31, 32, true);
	g.set_edge(31, 33, true);
	g.set_edge(31, 34, true);
	g.set_edge(35, 39, true);
	g.set_edge(35, 40, true);
	g.set_edge(35, 41, true);
	g.set_edge(35, 66, true);
	g.set_edge(36, 39, true);
	g.set_edge(36, 40, true);
	g.set_edge(36, 41, true);
	g.set_edge(37, 39, true);
	g.set_edge(37, 40, true);
	g.set_edge(37, 41, true);
	g.set_edge(38, 39, true);
	g.set_edge(38, 40, true);
	g.set_edge(38, 41, true);
	g.set_edge(38, 42, true);
	g.set_edge(42, 46, true);
	g.set_edge(42, 47, true);
	g.set_edge(42, 48, true);
	g.set_edge(43, 46, true);
	g.set_edge(43, 47, true);
	g.set_edge(43, 48, true);
	g.set_edge(44, 46, true);
	g.set_edge(44, 47, true);
	g.set_edge(44, 48, true);
	g.set_edge(45, 46, true);
	g.set_edge(45, 47, true);
	g.set_edge(45, 48, true);
	g.set_edge(45, 49, true);
	g.set_edge(49, 53, true);
	g.set_edge(49, 54, true);
	g.set_edge(49, 55, true);
	g.set_edge(50, 53, true);
	g.set_edge(50, 54, true);
	g.set_edge(50, 55, true);
	g.set_edge(51, 53, true);
	g.set_edge(51, 54, true);
	g.set_edge(51, 55, true);
	g.set_edge(52, 53, true);
	g.set_edge(52, 54, true);
	g.set_edge(52, 55, true);
	g.set_edge(52, 56, true);
	g.set_edge(56, 60, true);
	g.set_edge(56, 61, true);
	g.set_edge(56, 62, true);
	g.set_edge(57, 60, true);
	g.set_edge(57, 61, true);
	g.set_edge(57, 62, true);
	g.set_edge(58, 60, true);
	g.set_edge(58, 61, true);
	g.set_edge(58, 62, true);
	g.set_edge(59, 60, true);
	g.set_edge(59, 61, true);
	g.set_edge(59, 62, true);
	g.set_edge(59, 63, true);
	g.set_edge(63, 67, true);
	g.set_edge(63, 68, true);
	g.set_edge(63, 69, true);
	g.set_edge(64, 67, true);
	g.set_edge(64, 68, true);
	g.set_edge(64, 69, true);
	g.set_edge(65, 67, true);
	g.set_edge(65, 68, true);
	g.set_edge(65, 69, true);
	g.set_edge(66, 67, true);
	g.set_edge(66, 68, true);
	g.set_edge(66, 69, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_noperfectmatching_is_symmetric() {
	let mut g = Graph::empty(16);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 3, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 4, true);
	g.set_edge(3, 4, true);
	g.set_edge(4, 5, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 7, true);
	g.set_edge(6, 12, true);
	g.set_edge(6, 13, true);
	g.set_edge(7, 8, true);
	g.set_edge(7, 9, true);
	g.set_edge(8, 9, true);
	g.set_edge(8, 10, true);
	g.set_edge(8, 11, true);
	g.set_edge(9, 10, true);
	g.set_edge(9, 11, true);
	g.set_edge(10, 11, true);
	g.set_edge(12, 13, true);
	g.set_edge(12, 14, true);
	g.set_edge(12, 15, true);
	g.set_edge(13, 14, true);
	g.set_edge(13, 15, true);
	g.set_edge(14, 15, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_nonline_is_symmetric() {
	let mut g = Graph::empty(50);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(4, 6, true);
	g.set_edge(4, 7, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 7, true);
	g.set_edge(6, 7, true);
	g.set_edge(7, 8, true);
	g.set_edge(9, 11, true);
	g.set_edge(9, 12, true);
	g.set_edge(9, 13, true);
	g.set_edge(10, 11, true);
	g.set_edge(10, 12, true);
	g.set_edge(10, 13, true);
	g.set_edge(11, 12, true);
	g.set_edge(11, 13, true);
	g.set_edge(12, 13, true);
	g.set_edge(14, 15, true);
	g.set_edge(15, 16, true);
	g.set_edge(15, 17, true);
	g.set_edge(16, 17, true);
	g.set_edge(16, 18, true);
	g.set_edge(17, 18, true);
	g.set_edge(18, 19, true);
	g.set_edge(20, 21, true);
	g.set_edge(20, 22, true);
	g.set_edge(20, 23, true);
	g.set_edge(21, 22, true);
	g.set_edge(21, 23, true);
	g.set_edge(21, 24, true);
	g.set_edge(22, 23, true);
	g.set_edge(22, 24, true);
	g.set_edge(24, 25, true);
	g.set_edge(26, 27, true);
	g.set_edge(26, 28, true);
	g.set_edge(26, 29, true);
	g.set_edge(27, 28, true);
	g.set_edge(27, 29, true);
	g.set_edge(27, 30, true);
	g.set_edge(27, 31, true);
	g.set_edge(28, 29, true);
	g.set_edge(28, 30, true);
	g.set_edge(28, 31, true);
	g.set_edge(30, 31, true);
	g.set_edge(32, 34, true);
	g.set_edge(32, 35, true);
	g.set_edge(32, 36, true);
	g.set_edge(33, 34, true);
	g.set_edge(33, 35, true);
	g.set_edge(33, 37, true);
	g.set_edge(34, 35, true);
	g.set_edge(36, 37, true);
	g.set_edge(38, 39, true);
	g.set_edge(38, 40, true);
	g.set_edge(38, 43, true);
	g.set_edge(39, 40, true);
	g.set_edge(39, 41, true);
	g.set_edge(39, 42, true);
	g.set_edge(39, 43, true);
	g.set_edge(40, 41, true);
	g.set_edge(41, 42, true);
	g.set_edge(42, 43, true);
	g.set_edge(44, 45, true);
	g.set_edge(44, 46, true);
	g.set_edge(45, 46, true);
	g.set_edge(45, 47, true);
	g.set_edge(46, 47, true);
	g.set_edge(46, 48, true);
	g.set_edge(47, 48, true);
	g.set_edge(47, 49, true);
	g.set_edge(48, 49, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_octahedral_is_symmetric() {
	let mut g = Graph::empty(6);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(0, 5, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 3, true);
	g.set_edge(1, 4, true);
	g.set_edge(2, 4, true);
	g.set_edge(2, 5, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 5, true);
	g.set_edge(4, 5, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_petersen_is_symmetric() {
	let mut g = Graph::empty(10);

	g.set_edge(0, 1, true);
	g.set_edge(0, 4, true);
	g.set_edge(0, 5, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 6, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 7, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 8, true);
	g.set_edge(4, 9, true);
	g.set_edge(5, 7, true);
	g.set_edge(5, 8, true);
	g.set_edge(6, 8, true);
	g.set_edge(6, 9, true);
	g.set_edge(7, 9, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_robertson_is_symmetric() {
	let mut g = Graph::empty(19);

	g.set_edge(0, 1, true);
	g.set_edge(0, 4, true);
	g.set_edge(0, 15, true);
	g.set_edge(0, 18, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 8, true);
	g.set_edge(1, 12, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 6, true);
	g.set_edge(2, 17, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 11, true);
	g.set_edge(3, 14, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 9, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 12, true);
	g.set_edge(5, 16, true);
	g.set_edge(6, 7, true);
	g.set_edge(6, 10, true);
	g.set_edge(7, 8, true);
	g.set_edge(7, 14, true);
	g.set_edge(7, 18, true);
	g.set_edge(8, 9, true);
	g.set_edge(8, 16, true);
	g.set_edge(9, 10, true);
	g.set_edge(9, 13, true);
	g.set_edge(10, 11, true);
	g.set_edge(10, 15, true);
	g.set_edge(11, 12, true);
	g.set_edge(11, 18, true);
	g.set_edge(12, 13, true);
	g.set_edge(13, 14, true);
	g.set_edge(13, 17, true);
	g.set_edge(14, 15, true);
	g.set_edge(15, 16, true);
	g.set_edge(16, 17, true);
	g.set_edge(17, 18, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_smallestcyclicgroup_is_symmetric() {
	let mut g = Graph::empty(9);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(0, 4, true);
	g.set_edge(0, 5, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 3, true);
	g.set_edge(1, 7, true);
	g.set_edge(1, 8, true);
	g.set_edge(2, 5, true);
	g.set_edge(2, 6, true);
	g.set_edge(2, 7, true);
	g.set_edge(3, 8, true);
	g.set_edge(4, 5, true);
	g.set_edge(6, 7, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_tetrahedral_is_symmetric() {
	let mut g = Graph::empty(4);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 3, true);
	g.set_edge(2, 3, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_thomassen_is_symmetric() {
	let mut g = Graph::empty(34);

	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(0, 20, true);
	g.set_edge(1, 3, true);
	g.set_edge(1, 4, true);
	g.set_edge(1, 21, true);
	g.set_edge(2, 4, true);
	g.set_edge(2, 22, true);
	g.set_edge(3, 23, true);
	g.set_edge(4, 14, true);
	g.set_edge(5, 7, true);
	g.set_edge(5, 8, true);
	g.set_edge(5, 24, true);
	g.set_edge(6, 8, true);
	g.set_edge(6, 9, true);
	g.set_edge(6, 25, true);
	g.set_edge(7, 9, true);
	g.set_edge(7, 26, true);
	g.set_edge(8, 20, true);
	g.set_edge(9, 19, true);
	g.set_edge(10, 12, true);
	g.set_edge(10, 13, true);
	g.set_edge(10, 27, true);
	g.set_edge(11, 13, true);
	g.set_edge(11, 14, true);
	g.set_edge(11, 28, true);
	g.set_edge(12, 14, true);
	g.set_edge(12, 29, true);
	g.set_edge(13, 30, true);
	g.set_edge(15, 17, true);
	g.set_edge(15, 18, true);
	g.set_edge(15, 30, true);
	g.set_edge(16, 18, true);
	g.set_edge(16, 19, true);
	g.set_edge(16, 31, true);
	g.set_edge(17, 19, true);
	g.set_edge(17, 32, true);
	g.set_edge(18, 33, true);
	g.set_edge(20, 21, true);
	g.set_edge(20, 26, true);
	g.set_edge(21, 22, true);
	g.set_edge(22, 23, true);
	g.set_edge(23, 27, true);
	g.set_edge(24, 25, true);
	g.set_edge(24, 33, true);
	g.set_edge(25, 26, true);
	g.set_edge(27, 28, true);
	g.set_edge(28, 29, true);
	g.set_edge(29, 30, true);
	g.set_edge(30, 31, true);
	g.set_edge(31, 32, true);
	g.set_edge(32, 33, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_tutte_is_symmetric() {
	let mut g = Graph::empty(46);

	g.set_edge(0, 10, true);
	g.set_edge(0, 11, true);
	g.set_edge(0, 12, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 7, true);
	g.set_edge(1, 19, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 41, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 27, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 33, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 45, true);
	g.set_edge(6, 9, true);
	g.set_edge(6, 29, true);
	g.set_edge(7, 8, true);
	g.set_edge(7, 21, true);
	g.set_edge(8, 9, true);
	g.set_edge(8, 22, true);
	g.set_edge(9, 24, true);
	g.set_edge(10, 13, true);
	g.set_edge(10, 14, true);
	g.set_edge(11, 26, true);
	g.set_edge(11, 28, true);
	g.set_edge(12, 30, true);
	g.set_edge(12, 31, true);
	g.set_edge(13, 15, true);
	g.set_edge(13, 21, true);
	g.set_edge(14, 15, true);
	g.set_edge(14, 18, true);
	g.set_edge(15, 16, true);
	g.set_edge(16, 17, true);
	g.set_edge(16, 20, true);
	g.set_edge(17, 18, true);
	g.set_edge(17, 23, true);
	g.set_edge(18, 24, true);
	g.set_edge(19, 25, true);
	g.set_edge(19, 40, true);
	g.set_edge(20, 21, true);
	g.set_edge(20, 22, true);
	g.set_edge(22, 23, true);
	g.set_edge(23, 24, true);
	g.set_edge(25, 26, true);
	g.set_edge(25, 38, true);
	g.set_edge(26, 34, true);
	g.set_edge(27, 28, true);
	g.set_edge(27, 39, true);
	g.set_edge(28, 34, true);
	g.set_edge(29, 30, true);
	g.set_edge(29, 44, true);
	g.set_edge(30, 35, true);
	g.set_edge(31, 32, true);
	g.set_edge(31, 35, true);
	g.set_edge(32, 33, true);
	g.set_edge(32, 42, true);
	g.set_edge(33, 43, true);
	g.set_edge(34, 36, true);
	g.set_edge(35, 37, true);
	g.set_edge(36, 38, true);
	g.set_edge(36, 39, true);
	g.set_edge(37, 42, true);
	g.set_edge(37, 44, true);
	g.set_edge(38, 40, true);
	g.set_edge(39, 41, true);
	g.set_edge(40, 41, true);
	g.set_edge(42, 43, true);
	g.set_edge(43, 45, true);
	g.set_edge(44, 45, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_uniquely3colorable_is_symmetric() {
	let mut g = Graph::empty(12);

	g.set_edge(0, 1, true);
	g.set_edge(0, 3, true);
	g.set_edge(0, 6, true);
	g.set_edge(0, 8, true);
	g.set_edge(1, 4, true);
	g.set_edge(1, 7, true);
	g.set_edge(1, 9, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 6, true);
	g.set_edge(2, 7, true);
	g.set_edge(2, 9, true);
	g.set_edge(2, 11, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 10, true);
	g.set_edge(4, 5, true);
	g.set_edge(4, 11, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 7, true);
	g.set_edge(5, 8, true);
	g.set_edge(5, 10, true);
	g.set_edge(8, 11, true);
	g.set_edge(9, 10, true);

	assert_eq!(g.search_asymmetry(), false);
}
#[test]
fn igraph_walther_is_asymmetric() {
	let mut g = Graph::empty(25);

	g.set_edge(0, 1, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 8, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 13, true);
	g.set_edge(3, 4, true);
	g.set_edge(3, 16, true);
	g.set_edge(4, 5, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 19, true);
	g.set_edge(6, 7, true);
	g.set_edge(6, 20, true);
	g.set_edge(7, 21, true);
	g.set_edge(8, 9, true);
	g.set_edge(8, 13, true);
	g.set_edge(9, 10, true);
	g.set_edge(9, 22, true);
	g.set_edge(10, 11, true);
	g.set_edge(10, 20, true);
	g.set_edge(11, 12, true);
	g.set_edge(13, 14, true);
	g.set_edge(14, 15, true);
	g.set_edge(14, 23, true);
	g.set_edge(15, 16, true);
	g.set_edge(15, 17, true);
	g.set_edge(17, 18, true);
	g.set_edge(18, 19, true);
	g.set_edge(18, 24, true);
	g.set_edge(20, 24, true);
	g.set_edge(22, 23, true);
	g.set_edge(23, 24, true);

	assert_eq!(g.search_asymmetry(), true);
}
#[test]
fn igraph_zachary_is_symmetric() {
	let mut g = Graph::empty(34);

	g.set_edge(0, 1, true);
	g.set_edge(0, 2, true);
	g.set_edge(0, 3, true);
	g.set_edge(0, 4, true);
	g.set_edge(0, 5, true);
	g.set_edge(0, 6, true);
	g.set_edge(0, 7, true);
	g.set_edge(0, 8, true);
	g.set_edge(0, 10, true);
	g.set_edge(0, 11, true);
	g.set_edge(0, 12, true);
	g.set_edge(0, 13, true);
	g.set_edge(0, 17, true);
	g.set_edge(0, 19, true);
	g.set_edge(0, 21, true);
	g.set_edge(0, 31, true);
	g.set_edge(1, 2, true);
	g.set_edge(1, 3, true);
	g.set_edge(1, 7, true);
	g.set_edge(1, 13, true);
	g.set_edge(1, 17, true);
	g.set_edge(1, 19, true);
	g.set_edge(1, 21, true);
	g.set_edge(1, 30, true);
	g.set_edge(2, 3, true);
	g.set_edge(2, 7, true);
	g.set_edge(2, 8, true);
	g.set_edge(2, 9, true);
	g.set_edge(2, 13, true);
	g.set_edge(2, 27, true);
	g.set_edge(2, 28, true);
	g.set_edge(2, 32, true);
	g.set_edge(3, 7, true);
	g.set_edge(3, 12, true);
	g.set_edge(3, 13, true);
	g.set_edge(4, 6, true);
	g.set_edge(4, 10, true);
	g.set_edge(5, 6, true);
	g.set_edge(5, 10, true);
	g.set_edge(5, 16, true);
	g.set_edge(6, 16, true);
	g.set_edge(8, 30, true);
	g.set_edge(8, 32, true);
	g.set_edge(8, 33, true);
	g.set_edge(9, 33, true);
	g.set_edge(13, 33, true);
	g.set_edge(14, 32, true);
	g.set_edge(14, 33, true);
	g.set_edge(15, 32, true);
	g.set_edge(15, 33, true);
	g.set_edge(18, 32, true);
	g.set_edge(18, 33, true);
	g.set_edge(19, 33, true);
	g.set_edge(20, 32, true);
	g.set_edge(20, 33, true);
	g.set_edge(22, 32, true);
	g.set_edge(22, 33, true);
	g.set_edge(23, 25, true);
	g.set_edge(23, 27, true);
	g.set_edge(23, 29, true);
	g.set_edge(23, 32, true);
	g.set_edge(23, 33, true);
	g.set_edge(24, 25, true);
	g.set_edge(24, 27, true);
	g.set_edge(24, 31, true);
	g.set_edge(25, 31, true);
	g.set_edge(26, 29, true);
	g.set_edge(26, 33, true);
	g.set_edge(27, 33, true);
	g.set_edge(28, 31, true);
	g.set_edge(28, 33, true);
	g.set_edge(29, 32, true);
	g.set_edge(29, 33, true);
	g.set_edge(30, 32, true);
	g.set_edge(30, 33, true);
	g.set_edge(31, 32, true);
	g.set_edge(31, 33, true);
	g.set_edge(32, 33, true);

	assert_eq!(g.search_asymmetry(), false);
}
