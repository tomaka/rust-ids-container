use std::collections::hash_map::{self, HashMap};

pub struct IdsContainer<T> {
    data: HashMap<u32, T>,
    next_id: u32
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id {
    value: u32,
}

pub struct Iter<'a, T: 'a> {
    iter: hash_map::Iter<'a, u32, T>,
}

pub struct IterMut<'a, T: 'a> {
    iter: hash_map::IterMut<'a, u32, T>,
}

impl<T> IdsContainer<T> {
    pub fn new() -> IdsContainer<T> {
        IdsContainer {
            data: HashMap::new(),
            next_id: 1,
        }
    }

    /// Returns an iterator for all the keys and values in the container.
    pub fn get<'a>(&'a self, id: &Id) -> Option<&'a T> {
        self.data.get(&id.value)
    }

    /// Returns an iterator for all the keys and values in the container.
    pub fn get_mut<'a>(&'a mut self, id: &Id) -> Option<&'a mut T> {
        self.data.get_mut(&id.value)
    }

    /// Returns an iterator for all the keys and values in the container.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            iter: self.data.iter()
        }
    }

    /// Returns an iterator for all the keys and values in the container.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            iter: self.data.iter_mut()
        }
    }

    /// Returns an iterator for all the values in the container.
    pub fn values(&self) -> hash_map::Values<u32, T> {
        self.data.values()
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the container is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Inserts a new element and returns its identifier.
    pub fn insert(&mut self, value: T) -> Id {
        let new_id = self.next_id.clone();
        self.next_id += 1;

        self.data.insert(new_id, value);

        Id {
            value: new_id
        }
    }

    /// Removes an element from the container.
    ///
    /// # Panic
    ///
    /// Panics if the `id` was already removed.
    pub fn remove(&mut self, id: Id) {
        self.data.remove(&id.value).unwrap();
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = (Id, &'a T);

    fn next(&mut self) -> Option<(Id, &'a T)> {
        self.iter.next().map(|(id, value)| (Id { value: id.clone() }, value))
    }
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = (Id, &'a mut T);

    fn next(&mut self) -> Option<(Id, &'a mut T)> {
        self.iter.next().map(|(id, value)| (Id { value: id.clone() }, value))
    }
}
