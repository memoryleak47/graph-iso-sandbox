use crate::*;

pub struct Springs;

// Data[x] is the vector of x.
type Data = MMap<Symbol, MMap<Symbol, R64>>;

impl Simulation for Springs {
    type Footprint = Vec<Vec<R64>>;

    fn simulate(g: &SymbolGraph) -> Self::Footprint {
        let mut d = init_data(g);
        let n = g.nodes.len();
        for _ in 0..(n*n) {
            d = step(g, d);
        }
        unorder(d)
    }
}

fn unorder(d: Data) -> Vec<Vec<R64>> {
    d.values()
     .map(|x| x.values().sorted())
     .sorted()
}

type Force = MMap<Symbol, R64>;

fn sum_forces(g: &SymbolGraph, input: MSet<Force>) -> Force {
    g.nodes.to_map(|i| input.map(|f| f.idx(i)).sum())
}

fn step(g: &SymbolGraph, d: Data) -> Data {
    let out: MMap</*to*/ Symbol, MMap</*dimension*/ Symbol, R64>> =
        g.nodes.to_map(|to|
            sum_forces(g, g.nodes.map(|from|
                compute_force(from, to, g, &d)
            ))
        );
    out
}

// computes the force caused by `from`, applied to `to`.
fn compute_force(from: Symbol, to: Symbol, g: &SymbolGraph, d: &Data) -> Force {
    // creates the base-force that keeps the old position, if no other forces apply.
    if from == to {
        return d.idx(from);
    }

    if !g.edges.contains(&(from, to)) {
        return g.nodes.to_map(|_| r64(0.0));
    }

    // to - from
    let delta: MMap<Symbol, R64> = g.nodes.to_map(|n| d.idx(to).idx(n) - d.idx(from).idx(n));

    // ||to - from||
    let dist: R64 = delta.values().map(|x| x * x).sum().sqrt();
    let offset = dist - 1.0;

   delta.map_values(|x| x * -offset * r64(0.001))
}

fn init_data(g: &SymbolGraph) -> Data {
    g.nodes.to_map(|x|
        g.nodes.to_map(|y| {
            if x == y { r64(1.0) }
            else { r64(0.0) }
        })
    )
}
