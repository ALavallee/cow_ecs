use std::any::{TypeId};
use std::collections::HashMap;
use crate::component::comp_lock::{CompLock, CompLockAny};
use crate::component::component::{Component, ComponentAny};
use crate::entity::entity::EntityId;

pub struct CompManager {
    components: HashMap<TypeId, Box<dyn CompLockAny>>,
}

impl CompManager {
    pub fn new() -> Self {
        Self { components: HashMap::new() }
    }

    pub fn query<T: Component + 'static>(&self) -> Option<&CompLock<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.components.get(&type_id).unwrap();
        storage.as_any().downcast_ref::<CompLock<T>>()
    }

    pub fn query_mut<T: Component + 'static>(&mut self) -> Option<&mut CompLock<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.components.get_mut(&type_id).unwrap();
        storage.as_any_mut().downcast_mut::<CompLock<T>>()
    }

    pub fn register<T: Component + 'static>(&mut self) {
        let id = TypeId::of::<T>();
        if self.components.get(&id).is_none() {
            self.components.insert(id, Box::new(CompLock::<T>::new()));
        }
    }

    pub fn insert_by_comp_id(&mut self, type_id: TypeId, entity_id: EntityId, comp: Box<dyn ComponentAny>) {
        let storage = self.components.get_mut(&type_id).unwrap();
        storage.add_any(entity_id, comp);
    }

    pub fn remove(&mut self, entity_id: EntityId) {
        for comp in self.components.values_mut() {
            comp.as_mut().remove_any(entity_id)
        }
    }
}