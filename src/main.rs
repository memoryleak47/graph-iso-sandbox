pub use noisy_float::types::{R64, r64};
pub use noisy_float::prelude::Float;

mod graph;
pub use graph::*;

mod symbol;
pub use symbol::*;

mod simulation;
pub use simulation::*;

mod collections;
pub use collections::*;

mod algo;
pub use algo::*;

mod iso;
pub use iso::*;

fn main() {
    let arg = |i| std::env::args().nth(i)
                                  .map(|x| x.to_string())
                                  .unwrap_or(format!("{}.g", i));
    let g1 = Graph::parse_file(&arg(1));
    let g2 = Graph::parse_file(&arg(2));
    dbg!(is_isomorphic(&g1, &g2));
}
