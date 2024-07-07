use std::cmp::Ordering;

pub struct Map<I: Eq, O: Eq> {
    data: Vec<(I, O)>,
}

pub struct Set<T: Eq> {
    data: Vec<T>,
}

pub struct MultiSet<T: Eq> {
    data: Vec<T>,
}

impl<T: Eq> Set<T> {
    pub fn new() -> Self {
        Set { data: Vec::new() }
    }

    pub fn insert(&mut self, t: T) {
        if !self.data.contains(&t) {
            self.data.push(t);
        }
    }

    pub fn contains(&self, t: &T) -> bool {
        self.data.contains(t)
    }

    pub fn remove(&mut self, t: &T) {
        self.data.retain(|x| x != t);
    }

    pub fn try_sort_by<U: Ord>(&self, f: &impl Fn(&T) -> U) -> Option<Vec<T>> where T: Clone {
        let mut l = self.data.clone();
        l.sort_by_key(f);
        let n = l.len();

        for i in 0..n-1 {
            let a = &l[i];
            let b = &l[i+1];

            // If two elements aren't strictly orderable, they have to be equal.
            // Otherwise different input isomorphisms might lead to different orderings in the resulting Vec.
            if matches!(f(a).cmp(&f(b)), Ordering::Equal) && a != b {
                return None;
            }
        }

        Some(l)
    }
}
