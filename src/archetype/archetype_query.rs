use crate::archetype::archetype_iter::{ArchetypeQueryIter, ArchetypeQueryIterMut};
use crate::component::component::Component;
use crate::entity::entity::EntityId;

pub struct ArchetypeQuery<'a, T: Component + 'static> {
    indices: Vec<&'a Vec<EntityId>>,
    storages: Vec<&'a Vec<T>>,
}

impl<'a, T: Component + 'static> ArchetypeQuery<'a, T> {
    pub fn new(indices: Vec<&'a Vec<EntityId>>,
               storages: Vec<&'a Vec<T>>) -> Self {
        Self { indices, storages }
    }

    pub fn iter(&self) -> ArchetypeQueryIter<T> {
        ArchetypeQueryIter::new(self)
    }

    pub fn indices(&self) -> &Vec<&'a Vec<EntityId>> {
        &self.indices
    }

    pub fn storage(&self) -> &Vec<&'a Vec<T>> {
        &self.storages
    }

    pub fn query(&self, entity_query: EntityId) -> Option<&T> {
        for (i, indices) in self.indices.iter().enumerate() {
            for (j, entity) in indices.iter().enumerate() {
                if *entity == entity_query {
                    return Some(&self.storages[i][j]);
                }
            }
        }

        None
    }
}

pub struct ArchetypeQueryMut<'a, T: Component + 'static> {
    indices: Vec<&'a Vec<EntityId>>,
    storages: Vec<&'a mut Vec<T>>,
}

impl<'a, T: Component + 'static> ArchetypeQueryMut<'a, T> {
    pub fn new(indices: Vec<&'a Vec<EntityId>>, storages: Vec<&'a mut Vec<T>>) -> Self {
        Self { indices, storages }
    }

    pub fn iter_mut(&mut self) -> ArchetypeQueryIterMut<'a, T> {
        ArchetypeQueryIterMut::new(self)
    }

    pub fn indices(&self) -> &Vec<&'a Vec<EntityId>> {
        &self.indices
    }

    pub fn storages(&mut self) -> &mut Vec<&'a mut Vec<T>> {
        &mut self.storages
    }

    pub fn query(&self, entity_query: EntityId) -> Option<&T> {
        for (i, indices) in self.indices.iter().enumerate() {
            for (j, entity) in indices.iter().enumerate() {
                if *entity == entity_query {
                    return Some(&self.storages[i][j]);
                }
            }
        }

        None
    }

    pub fn query_mut(&mut self, entity_query: EntityId) -> Option<&mut T> {
        for (i, indices) in self.indices.iter().enumerate() {
            for (j, entity) in indices.iter().enumerate() {
                if *entity == entity_query {
                    return Some(&mut self.storages[i][j]);
                }
            }
        }

        None
    }
}