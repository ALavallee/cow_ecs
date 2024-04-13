use std::collections::hash_map::{Iter, Keys};
use std::collections::HashMap;
use std::hash::Hash;

pub struct SparseSet<I, T>
    where
        I: Hash + Eq + Copy, // Ensure `I` can be used as a key in `HashMap`
{
    sparse: HashMap<I, usize>,
    indices: Vec<I>,
    dense: Vec<T>,
}

impl<I, T> SparseSet<I, T>
    where
        I: Hash + Eq + Copy {
    pub fn new() -> Self {
        Self { sparse: HashMap::new(), indices: vec![], dense: Vec::new() }
    }

    pub fn insert(&mut self, index: I, value: T) {
        // Check if the element already exists
        if let Some(&dense_index) = self.sparse.get(&index) {
            // If it exists, replace the value in the dense array<
            self.dense[dense_index] = value;
        } else {
            // If it's a new element, add it to the dense array and update the sparse map
            self.dense.push(value);
            self.indices.push(index);
            let dense_index = self.dense.len() - 1;
            self.sparse.insert(index, dense_index);
        }
    }

    pub fn remove(&mut self, index: I) {
        // Check if the element exists
        if let Some(dense_index) = self.sparse.remove(&index) {
            // If it's not the last element, swap it with the last one
            if dense_index < self.dense.len() - 1 {
                // Swap the last element with the one to remove
                let last_element = self.dense.pop().unwrap();
                let last_index = self.indices.pop().unwrap();
                self.dense[dense_index] = last_element;
                self.indices[dense_index] = last_index;

                // Update the index of the swapped element in the sparse map
                // We find the key corresponding to the last element's index
                let key_to_update = self.sparse.iter().find_map(|(key, &value)| {
                    if value == self.dense.len() { Some(key) } else { None }
                }).unwrap();

                self.sparse.insert(*key_to_update, dense_index);
            } else {
                // If it's the last element, just pop it
                self.dense.pop();
                self.indices.pop();
            }
        }
    }

    pub fn get(&self, index: I) -> Option<&T> {
        if let Some(index) = self.sparse.get(&index) {
            return self.dense.get(*index);
        }
        None
    }

    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        if let Some(index) = self.sparse.get_mut(&index) {
            return self.dense.get_mut(*index);
        }
        None
    }

    pub fn iter(&self) -> SparseArrayIter<I, T> {
        SparseArrayIter {
            dense: &self.dense,
            indices: &self.indices,
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> SparseArrayIterMut<I, T> {
        SparseArrayIterMut {
            dense: &mut self.dense,
            indices: &mut self.indices,
            index: 0,
        }
    }
}

pub struct SparseArrayIter<'a, I: Hash + Eq + Copy, T> {
    dense: &'a Vec<T>,
    indices: &'a Vec<I>,
    index: usize,
}

impl<'a, I, T> Iterator for SparseArrayIter<'a, I, T>
    where
        I: Hash + Eq + Copy,
{
    type Item = (I, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.dense.len() {
            let value = &self.dense[self.index];
            let key = self.indices[self.index];
            self.index += 1;
            Some((key, value))
        } else {
            None
        }
    }
}


pub struct SparseArrayIterMut<'a, I: Hash + Eq + Copy, T> {
    dense: &'a mut Vec<T>,
    indices: &'a Vec<I>,
    index: usize,
}

impl<'a, I, T> Iterator for SparseArrayIterMut<'a, I, T>
    where
        I: Hash + Eq + Copy,
{
    type Item = (I, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.dense.len() {
            // unsafe block for performance
            unsafe {
                let key = self.indices.get_unchecked(self.index);
                // unsafe block to obtain a mutable reference to a value
                let value = unsafe {
                    &mut *(self.dense.as_mut_ptr().add(self.index))
                };
                self.index += 1;
                Some((*key, value))
            }
        } else {
            None
        }
    }
}

pub struct SparseArrayIntersectionIter<'a, I, T, X>
    where
        I: Hash + Eq + Copy,
{
    iter_a: SparseArrayIter<'a, I, T>,
    iter_b: SparseArrayIter<'a, I, X>,
}

impl<'a, I, T, X> SparseArrayIntersectionIter<'a, I, T, X>
    where
        I: Hash + Eq + Copy,
{
    pub fn new(iter_a: SparseArrayIter<'a, I, T>, iter_b: SparseArrayIter<'a, I, X>) -> Self {
        Self { iter_a, iter_b }
    }
}

impl<'a, I, T, X> Iterator for SparseArrayIntersectionIter<'a, I, T, X>
    where
        I: Hash + Eq + Copy,
{
    type Item = (I, &'a T, &'a X);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct SparseArrayIntersectionMutIter<'a, I, T, X>
    where
        I: Hash + Eq + Copy,
{
    indices_a: &'a Vec<I>,
    sparse_b: &'a HashMap<I, usize>,
    dense_a: &'a mut Vec<T>,
    dense_b: &'a mut Vec<X>,
    index: usize,
}

impl<'a, I, T, X> SparseArrayIntersectionMutIter<'a, I, T, X>
    where
        I: Hash + Eq + Copy,
{
    pub fn new(a: &'a mut SparseSet<I, T>, b: &'a mut SparseSet<I, X>) -> Self {
        Self {
            indices_a: &a.indices,
            sparse_b: &b.sparse,
            dense_a: &mut a.dense,
            dense_b: &mut b.dense,
            index: 0,
        }
    }
}

impl<'a, I, T, X> Iterator for SparseArrayIntersectionMutIter<'a, I, T, X>
    where
        I: Hash + Eq + Copy,
{
    type Item = (I, &'a mut T, &'a mut X);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.dense_b.len() {
            let a_index = self.indices_a[self.index];  // Copy index value, usize is Copy

            unsafe {
                // Obtain a mutable reference to dense_a value safely assuming unique access
                let a_value = &mut *(self.dense_a.as_mut_ptr().add(self.index));

                // Check if there is a corresponding entry in sparse_b
                if let Some(b_index) = self.sparse_b.get(&a_index) {
                    // Obtain a mutable reference to dense_b value safely assuming unique access
                    let b_value = &mut *(self.dense_b.as_mut_ptr().add(*b_index));

                    self.index += 1;  // Safe to increment index here
                    return Some((a_index, a_value, b_value));
                }
            }
            self.index += 1;  // Increment if no valid b_index found
        }
        None
    }
}
