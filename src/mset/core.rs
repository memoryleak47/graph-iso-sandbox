// A multi set.
#[derive(Clone)]
pub struct MSet<T> {
    data: Vec<T>,
}

// This is the core API of multiset.
// You should never be able to extract an arbitrary asymmetry from the elements of `data` from it.
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

    pub fn map<U>(&self, f: impl Fn(T) -> U) -> MSet<U>
        where T: Clone
    {
        self.data.iter().cloned().map(f).collect()
    }

    pub fn filter(&self, f: impl Fn(&T) -> bool) -> MSet<T>
        where T: Clone
    {
        self.data.iter().cloned().filter(f).collect()
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
