use crate::*;

// Data[x] is the vector of x.
pub type Data = Vec<Vec<f64>>;

pub fn springs(g: &Graph) -> Data {
    let mut d = init_data(g);
    for _ in g.nodes() {
        for _ in g.nodes() {
            step(g, &mut d);
        }
    }
    d
}

type Force = Vec<f64>;

fn step(g: &Graph, d: &mut Data) {
    // forces[i] is the resulting force we will apply to node i.
    let mut forces: Vec<Force> = Vec::new();

    // compute forces
    for i in g.nodes() {
        // fs stores all forces that will be applied to i.
        let mut fs = Vec::new();
        for &j in g.neighbours(i) {
            fs.push(compute_force(i, j, d));
        }
        let force = sum(fs, g.n());
        forces.push(force);
    }

    // apply forces
    for (i, force) in forces.into_iter().enumerate() {
        add_to(&mut d[i], &force);
    }
}

// computes the force applied on i by j.
fn compute_force(i: Node, j: Node, d: &Data) -> Force {
    let i = &d[i];
    let j = &d[j];

    // i - j
    let delta: Vec<f64> = i.iter().zip(j.iter()).map(|(x, y)| x-y).collect();

    // ||i-j||
    let dist = sum_fair(delta.iter().map(|x| x*x).collect()).sqrt();
    let offset = dist - 1.0;

    let force = delta.iter().map(|x| x * -offset * 0.001).collect();
    force
}

fn init_data(g: &Graph) -> Data {
    let mut d = Data::new();
    for _ in g.nodes() {
        d.push(vec![0.0; g.n()]);
    }
    for i in g.nodes() {
        d[i][i] = 1.0;
    }

    d
}

fn add_to(a: &mut [f64], b: &[f64]) {
    for i in 0..a.len() {
        a[i] += b[i];
    }
}

// sum([v1, v2, v3, ..., vn]) = v1 + v2 + ... + vn.
fn sum(forces: Vec<Force>, n: usize) -> Force {
    let mut out = vec![0.0; n];
    for i in 0..n {
        let mut v = Vec::new();
        for f in &forces {
            v.push(f[i]);
        }
        out[i] = sum_fair(v);
    }
    out
}

fn sum_fair(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v.into_iter().sum()
}
