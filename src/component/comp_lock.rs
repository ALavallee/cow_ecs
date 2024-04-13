use std::any::Any;
use std::sync::{Arc, RwLock};
use crate::component::comp_storage::{CompStorage, CompStorageAny};
use crate::component::component::Component;
use crate::entity::entity::EntityId;

pub trait CompLockAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn remove_any(&mut self, entity_id: EntityId);
}

pub struct CompLock<T : Component> {
    storage: Arc<RwLock<CompStorage<T>>>,
}

impl<T: Component + 'static> CompLock<T> {
    pub fn new() -> Self {
        Self { storage: Arc::new(RwLock::new(CompStorage::<T>::new())) }
    }

    pub fn add(&mut self, entity_id: EntityId, comp: T) {
        let mut storage = self.storage.write().unwrap(); // Acquire a write lock
        storage.add(entity_id, comp);
    }

    pub fn remove(&mut self, entity_id: EntityId) {
        self.storage.write().unwrap().remove(entity_id)
    }

    pub fn storage(&self) -> &Arc<RwLock<CompStorage<T>>> {
        &self.storage
    }
}


impl<T: Component + 'static> CompLockAny for CompLock<T> {
    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove_any(&mut self, entity_id: EntityId) {
        self.remove(entity_id)
    }
}