use crate::*;

pub trait Simulation {
    type Footprint: Eq;
    fn simulate(g: &SymbolGraph) -> Self::Footprint;
}

#[derive(Debug)]
pub enum FootprintResult<T> {
    Equal(T),
    Different(T, T),
}

pub fn witness_non_isomorphism<S: Simulation>(g1: &Graph, g2: &Graph) -> FootprintResult<S::Footprint> {
    let g1 = SymbolGraph::from(g1);
    let g2 = SymbolGraph::from(g2);
    let f1 = S::simulate(&g1);
    let f2 = S::simulate(&g2);
    if f1 != f2 {
        FootprintResult::Different(f1, f2)
    } else {
        FootprintResult::Equal(f1)
    }
}
