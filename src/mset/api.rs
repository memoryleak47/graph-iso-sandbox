use crate::*;
use std::borrow::Borrow;

pub type MMap<K, V> = MSet<(K, V)>;

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

    pub fn len(&self) -> usize {
        self.map_ref(|_| 1).sum()
    }

    pub fn to_map<V>(&self, f: impl Fn(T) -> V) -> MMap<T, V>
        where T: Clone
    {
        self.map(|k| (k.clone(), f(k)))
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

impl<K, V> MMap<K, V>
    where K: Eq + Clone,
          V: Eq + Clone
{
    pub fn get<B>(&self, b: B) -> Option<V>
        where B: Borrow<K>
    {
        let k = b.borrow();
        self.filter(|(k2, _)| k2 == k)
            .map(|(_, v)| v)
            .get_unique()
    }

    pub fn idx<B>(&self, b: B) -> V
        where B: Borrow<K>
    {
        self.get(b).unwrap()
    }

    pub fn keys(&self) -> MSet<K> {
        self.map(|(k, _)| k)
    }

    pub fn values(&self) -> MSet<V> {
        self.map(|(_, v)| v)
    }

    pub fn map_values<V2>(&self, f: impl Fn(V) -> V2) -> MMap<K, V2> {
        self.map(|(k, v)| (k, f(v)))
    }
}
