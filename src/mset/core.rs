// A multi set.
// Two MSets with a different permutation of the same elements should never be able to be distinguished.
// Invariants for that:
// - If a == b (by the Eq trait), then a and b are indistuishable.
// - Each Ord implementation is truly a total order
// - Looking at the raw bits of a datastructure using unsafe is forbidden. Similarly pointer to int casts (and related things) are forbidden.
#[derive(Clone)]
pub struct MSet<T> {
    data: Vec<T>,
}

// This is the core API of multiset.
impl<T> MSet<T> {
    pub fn new() -> Self {
        MSet { data: Vec::new() }
    }

    pub fn insert(&mut self, t: T) {
        self.data.push(t);
    }

    pub fn contains(&self, t: &T) -> bool
        where T: Eq
    {
        self.data.contains(t)
    }

    pub fn map_ref<U>(&self, f: impl Fn(&T) -> U) -> MSet<U>
    {
        self.data.iter().map(f).collect()
    }

    pub fn retain(&mut self, f: impl Fn(&T) -> bool) {
        self.data.retain(f);
    }

    pub fn union(&self, other: &MSet<T>) -> MSet<T>
        where T: Clone
    {
        let mut out = MSet::new();
        for x in &self.data { out.insert(x.clone()); }
            for x in &other.data { out.insert(x.clone()); }
        out
    }

    pub fn dedup(&self) -> MSet<T>
        where T: Clone + Eq
    {
        let mut out = MSet::new();
        for x in &self.data {
            if !out.contains(x) {
                out.insert(x.clone());
            }
        }
        out
    }

    pub fn try_sort_by_key<U>(&self, f: impl Fn(&T) -> U) -> Option<Vec<T>>
        where T: Clone + Eq,
              U: Ord
    {
        use std::cmp::Ordering;

        let mut l = self.data.clone();
        l.sort_by_key(|x| f(x));
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

    pub fn group_by<U>(&self, f: impl Fn(&T) -> U) -> MSet<(U, MSet<T>)>
        where T: Clone,
              U: Eq
    {
        let mut out = MSet::new();
        for x in &self.data {
            let u = f(x);
            let i = out.data.iter().position(|(u2, _)| u2 == &u).unwrap_or_else(|| {
                out.data.push((u, MSet::new()));
                out.data.len()-1
            });
            out.data[i].1.insert((*x).clone());
        }
        out
    }
}

impl<T> MSet<MSet<T>> {
    pub fn flatten(&self) -> MSet<T>
        where T: Clone
    {
        let mut out = MSet::new();
        for x in &self.data {
            for y in &x.data {
                out.insert(y.clone());
            }
        }
        out
    }
}

impl<A> FromIterator<A> for MSet<A> {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item=A>
    {
        MSet { data: iter.into_iter().collect() }
    }
}

impl<T> PartialEq<MSet<T>> for MSet<T>
    where T: PartialEq
{
    fn eq(&self, other: &MSet<T>) -> bool {
        let mut b: Vec<&T> = other.data.iter().collect();
        for x in &self.data[..] {
            let Some(i) = b.iter().position(|y| y == &x) else { return false };
            b.remove(i);
        }

        b.is_empty()
    }
}

impl<T> Eq for MSet<T> where T: Eq {}
