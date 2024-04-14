use std::sync::{Arc, RwLock};
use crate::entity::entity::EntityId;
use crate::entity::entity_manager::EntityManager;

pub struct EntityLock {
    manager: Arc<RwLock<EntityManager>>,
}

impl EntityLock {
    pub fn new() -> Self {
        Self { manager: Arc::new(RwLock::new(EntityManager::new())) }
    }

    pub fn create(&mut self) -> EntityId {
        self.manager.write().unwrap().generate()
    }

    pub fn release(&mut self, entity_id: EntityId) {
        self.manager.write().unwrap().release(entity_id)
    }

    pub fn count(&self) -> usize {
        self.manager.read().unwrap().count()
    }

    pub fn manager(&self) -> &Arc<RwLock<EntityManager>> {
        &self.manager
    }
}