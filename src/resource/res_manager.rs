use std::any::TypeId;
use std::collections::HashMap;
use crate::resource::res_lock::{ResLock, ResLockAny};
use crate::resource::resource::Resource;

pub struct ResManager {
    components: HashMap<TypeId, Box<dyn ResLockAny>>,
}

impl ResManager {
    pub fn new() -> Self {
        Self { components: HashMap::new() }
    }

    pub fn set<T: Resource + 'static>(&mut self, res: T) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, Box::new(ResLock::<T>::new(res)));
    }

    pub fn query<T: Resource + 'static>(&self) -> Option<&ResLock<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.components.get(&type_id).unwrap();
        storage.as_any().downcast_ref::<ResLock<T>>()
    }

    pub fn query_mut<T: Resource + 'static>(&mut self) -> Option<&mut ResLock<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.components.get_mut(&type_id).unwrap();
        storage.as_any_mut().downcast_mut::<ResLock<T>>()
    }
}