use crate::archetype::archetype_iter::{ArchetypeQueryIter, ArchetypeQueryIterMut, ArchetypeQueryIterMutNoIndex};
use crate::archetype::archetype_query::{ArchetypeQuery, ArchetypeQueryMut};
use crate::commands::{EntityCommand, EntityCommands};
use crate::component::comp_storage::CompStorage;
use crate::component::component::Component;
use crate::entity::entity::EntityId;
use crate::resource::resource::Resource;

pub struct Comps<'a, T: Component + 'static> {
    query: ArchetypeQuery<'a, T>,
}

impl<'a, T: Component + 'static> Comps<'a, T> {
    pub fn new(query: ArchetypeQuery<'a, T>) -> Self {
        Self { query }
    }

    pub fn iter(&self) -> ArchetypeQueryIter<T> {
        ArchetypeQueryIter::new(&self.query)
    }

    pub fn query(&self, id: EntityId) -> Option<&T> {
        self.query.query(id)
    }
}

pub struct CompsMut<'a, T: Component + 'static> {
    query: ArchetypeQueryMut<'a, T>,
}

impl<'a, T: Component + 'static> CompsMut<'a, T> {
    pub fn new(query: ArchetypeQueryMut<'a, T>) -> Self {
        Self { query }
    }


    pub fn iter(&mut self) -> ArchetypeQueryIterMut<'a, T> {
        ArchetypeQueryIterMut::new(&mut self.query)
    }

    /*pub fn query_mut(&mut self, id: EntityId) -> Option<&mut T> {
        self.storage.query_mut(id)
    }*/

    /*pub fn query(&self, id: EntityId) -> Option<&T> {
        self.storage.query(id)
    }*/
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