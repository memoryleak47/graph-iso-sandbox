use crate::*;

// Converting nodes to Symbols makes them isomorphism invariant.
#[derive(PartialEq, Eq)]
pub struct Symbol(usize);

// A SymbolGraph is a graph, but its nodes have no order.
// Thus permuting the nodes in a SymbolGraph yields an indistinguishable SymbolGraph.
// That is useful for isomorphism-invariant algorithms.
pub struct SymbolGraph {
    pub nodes: MSet<Symbol>,
    pub edges: MSet<(Symbol, Symbol)>,
}

impl SymbolGraph {
    pub fn from(g: &Graph) -> Self {
        let n = g.n();
        let mut nodes = MSet::new();
        let mut edges = MSet::new();
        for i in 0..n {
            nodes.insert(Symbol(i));
            for &n in g.neighbours(i) {
                edges.insert((Symbol(i), Symbol(n)));
            }
        }
        Self { nodes, edges }
    }
}
