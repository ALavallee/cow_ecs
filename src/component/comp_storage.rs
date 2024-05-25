use std::any::{Any, TypeId};
use crate::component::component::{Component};

pub trait CompStorageAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn remove(&mut self, index: usize);

    fn transfer(&mut self, other: &mut Box<dyn CompStorageAny>, index: usize) -> (usize, usize);

    fn contained_type(&self) -> TypeId;

    fn duplicate(&self) -> Box<dyn CompStorageAny>;

    fn len(&self) -> usize;
}

pub struct CompStorage<T: Component> {
    components: Vec<T>,
}

impl<T: Component + 'static> CompStorage<T> {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn add(&mut self, comp: T) {
        self.components.push(comp);
    }

    pub fn update(&mut self, index: usize, comp: T) {
        self.components[index] = comp;
    }

    pub fn get(&self, index: usize) -> &T {
        &self.components[index]
    }

    pub fn components(&self) -> &Vec<T> {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut Vec<T> { &mut self.components }
}

impl<T: Component + 'static> CompStorageAny for CompStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove(&mut self, index: usize) {
        let last_index = self.components.len() - 1;
        self.components.swap(index, last_index);
        self.components.pop();
    }

    fn transfer(&mut self, dest: &mut Box<dyn CompStorageAny>, index: usize) -> (usize, usize) {
        // for transfer, we don't remove the component.
        // we need to switch the index with the last and then pop
        return if let Some(dest_casted) = dest.as_mut().as_any_mut().downcast_mut::<CompStorage<T>>() {
            let last_index = self.components.len() - 1;
            self.components.swap(index, last_index);
            let comp = self.components.pop().unwrap();
            dest_casted.components.push(comp);
            if self.components.is_empty() {
                (0, dest_casted.components.len() - 1)
            } else {
                (self.components.len() - 1, dest_casted.components.len() - 1)
            }
        } else {
            (0, 0)
        };
    }

    fn contained_type(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn duplicate(&self) -> Box<dyn CompStorageAny> {
        Box::new(CompStorage::<T>::new())
    }

    fn len(&self) -> usize { self.components.len() }
}

