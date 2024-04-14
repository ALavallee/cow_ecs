use std::collections::{HashSet};
use crate::entity::entity::{EntityId};

pub struct EntityManager {
    current: EntityId,
    frees: Vec<EntityId>,
    allocated: HashSet<EntityId>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self { current: 0, frees: vec![], allocated: HashSet::new() }
    }

    pub fn create(&mut self) -> EntityId {
        return if !self.frees.is_empty() {
            self.frees.pop().unwrap()
        } else {
            self.current += 1; // entity 0 must never exist
            let value = self.current;
            self.allocated.insert(value);
            value
        };
    }

    pub fn release(&mut self, id: EntityId) {
        self.allocated.remove(&id);
        self.frees.push(id)
    }

    pub fn count(&self) -> usize {
        self.allocated.len()
    }
}