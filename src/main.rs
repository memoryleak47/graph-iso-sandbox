mod graph;
use graph::*;

fn main() {
    let g = Graph::parse_file("input.txt");
    dbg!(&g);
}
