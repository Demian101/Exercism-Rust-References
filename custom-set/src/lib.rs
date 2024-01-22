use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: PartialEq + Eq + Hash> {
    // phantom: std::marker::PhantomData<T>,
    set: HashSet<T>,
}

impl<T> CustomSet<T> 
where
    T: PartialEq + Eq + Hash + Clone,
{
    pub fn new(_input: &[T]) -> Self {
        Self {
            // set: HashSet::new()
            set: _input.into_iter().cloned().collect(),
        }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.set.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.set.insert(_element);
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.set.is_subset(&_other.set)
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.set.is_disjoint(&_other.set)
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        Self {
            set: self.set.intersection(&_other.set).cloned().collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        Self {
            set: self.set.difference(&_other.set).cloned().collect(),
        }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        Self {
            set: self.set.union(&_other.set).cloned().collect(),
        }
    }
}
