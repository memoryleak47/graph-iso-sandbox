use crate::*;

pub struct NBody;

// Data[x] is the vector of x.
type Data = MMap<Symbol, MMap<Symbol, R64>>;
type PosData = Data;
type VelData = Data;

impl Simulation for NBody {
    type Footprint = (Vec<Vec<R64>>, Vec<Vec<R64>>);

    fn simulate(g: &SymbolGraph) -> Self::Footprint {
        let mut pos: PosData = init_pos_data(g);
        let mut vel: VelData = g.nodes.to_map(|_| g.nodes.to_map(|_| r64(0.0)));
        let n = g.nodes.len();
        for _ in 0..n {
            let new_pos = step_pos(g, &pos, &vel);
            let new_vel = step_vel(g, &pos, &vel);
            pos = new_pos;
            vel = new_vel;
        }
        (unorder(pos), unorder(vel))
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

fn step_pos(g: &SymbolGraph, pos: &PosData, vel: &VelData) -> PosData {
    let out: MMap</*to*/ Symbol, MMap</*dimension*/ Symbol, R64>> =
        g.nodes.to_map(|n| {
            let p = pos.idx(n);
            let v = vel.idx(n);
            let arr = [p, v].into_iter().collect();
            sum_forces(g, arr)
        });
    out
}

fn step_vel(g: &SymbolGraph, pos: &PosData, vel: &VelData) -> VelData {
    let out: MMap</*to*/ Symbol, MMap</*dimension*/ Symbol, R64>> =
        g.nodes.to_map(|to|
            sum_forces(g, g.nodes.map(|from|
                compute_force(from, to, g, pos, vel)
            ))
        );
    out
}

// computes the force caused by `from`, applied to `to`.
fn compute_force(from: Symbol, to: Symbol, g: &SymbolGraph, pos: &PosData, vel: &VelData) -> Force {
    if from == to {
        // This keeps the original speed in tact.
        return vel.idx(from);
    }

    // from - to
    let delta: MMap<Symbol, R64> = g.nodes.to_map(|n| pos.idx(from).idx(n) - pos.idx(to).idx(n));

    // ||from - to||
    let dist: R64 = delta.values().map(|x| x * x).sum().sqrt();
    let dist_sqr = dist * dist;

    delta.map_values(|x| x / dist_sqr)
}

fn init_pos_data(g: &SymbolGraph) -> PosData {
    g.nodes.to_map(|x|
        g.nodes.to_map(|y| {
            if x == y { r64(1.0) }
            else if g.edges.contains(&(x, y)) { r64(0.1) }
            else { r64(0.0) }
        })
    )
}
