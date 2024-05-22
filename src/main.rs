pub use noisy_float::types::{R64, r64};
pub use noisy_float::prelude::Float;

mod graph;
pub use graph::*;

mod algo;
pub use algo::*;

fn unorder(d: &mut Data) {
    for x in d.iter_mut() {
        x.sort();
    }
    d.sort();
}

fn is_isomorphic(g1: &Graph, g2: &Graph) -> bool {
    let mut d1 = springs(g1);
    let mut d2 = springs(g2);
    unorder(&mut d1);
    unorder(&mut d2);

    d1 == d2
}

fn main() {
    let g1 = Graph::parse_file("1.g");
    let g2 = Graph::parse_file("2.g");
    dbg!(is_isomorphic(&g1, &g2));
}
