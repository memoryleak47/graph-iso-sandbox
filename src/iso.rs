use crate::*;

pub fn is_isomorphic(g1: &Graph, g2: &Graph) -> bool {
    let mut d1 = springs(g1);
    let mut d2 = springs(g2);
    unorder(&mut d1);
    unorder(&mut d2);

    d1 == d2
}


fn unorder(d: &mut Data) {
    for x in d.iter_mut() {
        x.sort();
    }
    d.sort();
}
