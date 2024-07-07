use crate::*;

impl<T> MSet<T> {
    // panics if the sorting results in non-determinism.
    pub fn sort_by_key<U>(&self, f: impl Fn(&T) -> U) -> Vec<T>
        where T: Clone + Eq,
              U: Ord
    {
        self.try_sort_by_key(f).expect("Not all elements could be sorted!")
    }

    pub fn sort(&self) -> Vec<T>
        where T: Clone + Ord
    {
        self.sort_by_key(|x| x.clone())
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
