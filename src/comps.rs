use crate::commands::{EntityCommand, EntityCommands};
use crate::component::comp_storage::CompStorage;
use crate::component::component::Component;
use crate::data::sparse_set::{SparseArrayIntersectionIter, SparseArrayIntersectionMutIter, SparseArrayIter, SparseArrayIterMut};
use crate::entity::entity::EntityId;
use crate::resource::resource::Resource;

pub struct Comps<'a, T: Component> {
    storage: &'a CompStorage<T>,
}

impl<'a, T: Component + 'static> Comps<'a, T> {
    pub fn new(storage: &'a CompStorage<T>) -> Self {
        Self { storage }
    }

    pub fn query(&self, id: EntityId) -> Option<&T> {
        self.storage.query(id)
    }

    pub fn iter(&self) -> SparseArrayIter<EntityId, T> {
        self.storage.iter()
    }

    pub fn join<X: Component>(&self, other: &Comps<'a, X>) -> SparseArrayIntersectionIter<EntityId, T, X> {
        self.storage.iter_intersection(other.storage)
    }
}

pub struct CompsMut<'a, T: Component> {
    storage: &'a mut CompStorage<T>,
}

impl<'a, T: Component + 'static> CompsMut<'a, T> {
    pub fn new(storage: &'a mut CompStorage<T>) -> Self {
        Self { storage }
    }

    pub fn add(&mut self, entity_id: EntityId, comp: T) {
        self.storage.add(entity_id, comp)
    }

    pub fn query_mut(&mut self, id: EntityId) -> Option<&mut T> {
        self.storage.query_mut(id)
    }

    pub fn query(&self, id: EntityId) -> Option<&T> {
        self.storage.query(id)
    }

    pub fn iter_mut(&mut self) -> SparseArrayIterMut<EntityId, T> {
        self.storage.iter_mut()
    }

    pub fn iter(&self) -> SparseArrayIter<EntityId, T> {
        self.storage.iter()
    }

    pub fn join_mut<'b, X: Component>(&'b mut self, other: &'b mut CompsMut<X>) -> SparseArrayIntersectionMutIter<'b, EntityId, T, X>
    {
        self.storage.iter_mut_intersection(&mut other.storage)
    }

    pub fn join<X: Component>(&self, other: &Comps<'a, X>) -> SparseArrayIntersectionIter<EntityId, T, X> {
        self.storage.iter_intersection(other.storage)
    }
}

pub struct Res<'a, T: Resource> {
    resource: &'a T,
}

impl<'a, T: Resource> Res<'a, T> {
    pub fn new(resource: &'a T) -> Self {
        Self { resource }
    }

    pub fn get(&self) -> &T {
        &self.resource
    }
}


pub struct ResMut<'a, T: Resource> {
    resource: &'a mut T,
}

impl<'a, T: Resource> ResMut<'a, T> {
    pub fn new(resource: &'a mut T) -> Self {
        Self { resource }
    }

    pub fn get(&self) -> &T {
        &self.resource
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.resource
    }
}

pub struct Commands<'a> {
    commands: &'a mut EntityCommands,
}

impl<'a> Commands<'a> {
    pub fn new(commands: &'a mut EntityCommands) -> Self {
        Self { commands }
    }

    pub fn create(&mut self) -> &mut EntityCommand {
        self.commands.create()
    }

    pub fn remove(&mut self, entity_id: EntityId) {
        self.commands.release(entity_id)
    }
}