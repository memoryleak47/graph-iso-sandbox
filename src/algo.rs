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

fn step(g: &Graph, d: &mut Data) {
    todo!()
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
