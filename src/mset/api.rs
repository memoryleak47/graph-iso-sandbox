use crate::*;

impl<T> MSet<T> {
    // panics if the sorting results in non-determinism.
    pub fn sort_by(&self, f: impl Fn(&T, &T) -> Ordering) -> Vec<T>
        where T: Clone + Eq,
    {
        self.try_sort_by(f).expect("Not all elements could be sorted!")
    }

    // panics if the sorting results in non-determinism.
    pub fn sort_by_key<U>(&self, f: impl Fn(&T) -> U) -> Vec<T>
        where T: Clone + Eq,
              U: Ord
    {
        self.sort_by(|x, y| f(x).cmp(&f(y)))
    }

    pub fn sort(&self) -> Vec<T>
        where T: Clone + Ord
    {
        self.sort_by_key(|x| x.clone())
    }

    pub fn map<U>(&self, f: impl Fn(T) -> U) -> MSet<U>
        where T: Clone
    {
        self.map_ref(|x| f(x.clone()))
    }

    pub fn filter(&self, f: impl Fn(&T) -> bool) -> MSet<T>
        where T: Clone
    {
        let mut out = self.clone();
        out.retain(f);
        out
    }

    pub fn len(&self) -> usize
        where T: Clone
    {
        self.map(|_| 1).sum()
    }
}

impl<T> MSet<T>
    where T: Ord + Eq + Clone + std::iter::Sum
{
    pub fn sum(&self) -> T {
        self.sort().into_iter().sum::<T>()
    }
}

impl<T> MSet<T>
    where T: Ord + Eq + Clone + std::iter::Product
{
    pub fn product(&self) -> T {
        self.sort().into_iter().product::<T>()
    }
}
