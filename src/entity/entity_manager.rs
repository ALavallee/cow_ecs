use std::collections::{HashSet};
use crate::entity::entity::{EntityId};

pub struct EntityManager {
    current: EntityId,
    to_free: Vec<EntityId>,
    frees: Vec<EntityId>,
    allocated: HashSet<EntityId>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self { current: 0, to_free: vec![], frees: vec![], allocated: HashSet::new() }
    }

    pub fn generate(&mut self) -> EntityId {
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
        self.to_free.push(id)
    }

    pub fn count(&self) -> usize {
        self.allocated.len()
    }

    pub(crate) fn clear_releases(&mut self) -> Vec<EntityId> {
        for id in &self.to_free {
            self.allocated.remove(&id);
            self.frees.push(*id);
        }

        std::mem::replace(&mut self.frees, Vec::new())
    }
}