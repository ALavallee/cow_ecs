use std::any::Any;
use std::sync::{Arc, RwLock};
use crate::resource::resource::Resource;

pub trait ResLockAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ResLock<T: Resource> {
    res: Arc<RwLock<T>>,
}

impl<T: Resource + 'static> ResLock<T> {
    pub fn new(res: T) -> Self {
        Self { res: Arc::new(RwLock::new(res)) }
    }

    pub fn resource(&self) -> &Arc<RwLock<T>> {
        &self.res
    }
}

impl<T: Resource + 'static> ResLockAny for ResLock<T> {
    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}