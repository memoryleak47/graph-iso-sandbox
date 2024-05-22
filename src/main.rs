mod graph;
pub use graph::*;

mod algo;
pub use algo::*;

fn unorder(d: &mut Data) {
    use std::cmp::Ordering;

    let srt = |a: &f64, b: &f64| a.partial_cmp(b).unwrap();

    for x in d.iter_mut() {
        x.sort_by(srt);
    }

    d.sort_by(|v1, v2| {
        v1.iter().zip(v2.iter()).map(|(x, y)| srt(x, y))
        .fold(Ordering::Equal, Ordering::then)
    });
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
