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

mod springs;
pub use springs::*;

fn main() {
    let arg = |i| std::env::args().nth(i)
                                  .map(|x| x.to_string())
                                  .unwrap_or(format!("{}.g", i));
    let g1 = Graph::parse_file(&arg(1));
    let g2 = Graph::parse_file(&arg(2));
    dbg!(springs_iso_check(&g1, &g2));
}
