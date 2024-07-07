use crate::*;

impl<T> MSet<T> {
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
        self.sorted().into_iter().sum::<T>()
    }
}

impl<T> MSet<T>
    where T: Ord + Eq + Clone + std::iter::Product
{
    pub fn product(&self) -> T {
        self.sorted().into_iter().product::<T>()
    }
}
