pub use noisy_float::types::{R64, r64};
pub use noisy_float::prelude::Float;

mod graph;
pub use graph::*;

mod algo;
pub use algo::*;

mod iso;
pub use iso::*;

fn main() {
    let g1 = Graph::parse_file("1.g");
    let g2 = Graph::parse_file("2.g");
    dbg!(is_isomorphic(&g1, &g2));
}
