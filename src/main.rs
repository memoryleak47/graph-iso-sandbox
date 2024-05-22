mod graph;
pub use graph::*;

mod algo;
pub use algo::*;

fn main() {
    let g = Graph::parse_file("input.txt");
    let d = springs(&g);
}
