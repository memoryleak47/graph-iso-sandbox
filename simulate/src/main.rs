#![allow(unused_imports)]

pub use noisy_float::types::{R64, r64};
pub use noisy_float::prelude::Float;
use std::cmp::Ordering;

mod graph;
pub use graph::*;

mod symbol;
pub use symbol::*;

mod simulation;
pub use simulation::*;

mod mset;
pub use mset::*;

mod springs;
pub use springs::*;

mod nbody;
pub use nbody::*;

fn main() {
    let arg = |i| std::env::args().nth(i)
                                  .map(|x| x.to_string())
                                  .unwrap_or(format!("{}.g", i));
    let g1 = Graph::parse_file(&arg(1));
    let g2 = Graph::parse_file(&arg(2));
    dbg!(matches!(witness_non_isomorphism::<Springs>(&g1, &g2), FootprintResult::Different(..)));
    dbg!(matches!(witness_non_isomorphism::<NBody>(&g1, &g2), FootprintResult::Different(..)));
}
