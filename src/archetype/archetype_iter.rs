use crate::archetype::archetype_query::{ArchetypeQuery, ArchetypeQueryMut};
use crate::component::component::Component;
use crate::entity::entity::EntityId;

pub struct ArchetypeQueryIter<'a, T: 'static + Component> {
    query: &'a ArchetypeQuery<'a, T>,
    outer_index: usize,
    inner_index: usize,
}

impl<'a, T: Component> ArchetypeQueryIter<'a, T> {
    pub fn new(query: &'a ArchetypeQuery<'a, T>) -> Self {
        Self { query, outer_index: 0, inner_index: 0 }
    }
}

impl<'a, T: Component> Iterator for ArchetypeQueryIter<'a, T> {
    type Item = (EntityId, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.outer_index < self.query.indices().len() {
            let indices = self.query.indices()[self.outer_index];
            let storages = self.query.storage()[self.outer_index];

            if self.inner_index < indices.len() {
                let entity_id = indices[self.inner_index];
                let component = &storages[self.inner_index];
                self.inner_index += 1;
                return Some((entity_id, component));
            }

            self.outer_index += 1;
            self.inner_index = 0;
        }

        None
    }
}

pub struct ArchetypeQueryIterMut<'a, T: Component + 'static> {
    query: *mut ArchetypeQueryMut<'a, T>,
    outer_index: usize,
    inner_index: usize,
}

impl<'a, T: Component + 'static> ArchetypeQueryIterMut<'a, T> {
    pub fn new(query: &mut ArchetypeQueryMut<'a, T>) -> Self {
        Self {
            query,
            outer_index: 0,
            inner_index: 0,
        }
    }
}

impl<'a, T: Component> Iterator for ArchetypeQueryIterMut<'a, T> {
    type Item = (EntityId, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let query = &mut *self.query;

            while self.outer_index < query.indices().len() {
                let indices = query.indices()[self.outer_index];
                let storages = &mut *query.storages()[self.outer_index];

                if self.inner_index < indices.len() {
                    let entity_id = indices[self.inner_index];

                    let component = &mut *(&mut storages[self.inner_index] as *mut T);

                    self.inner_index += 1;
                    return Some((entity_id, component));
                }

                self.outer_index += 1;
                self.inner_index = 0;
            }

            None
        }
    }
}