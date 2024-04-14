use std::any::Any;
use crate::component::component::{Component, ComponentAny};
use crate::data::sparse_set::{SparseSet, SparseArrayIntersectionIter, SparseArrayIntersectionMutIter, SparseArrayIter, SparseArrayIterMut};
use crate::entity::entity::EntityId;

pub trait CompStorageAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn remove(&mut self, entity_id: EntityId);
}

pub struct CompStorage<T: Component> {
    sparse: SparseSet<EntityId, T>,
}

impl<T: Component + 'static> CompStorage<T> {
    pub fn new() -> Self {
        Self { sparse: SparseSet::new() }
    }

    pub(crate) fn query(&self, id: EntityId) -> Option<&T> {
        self.sparse.get(id)
    }

    pub(crate) fn query_mut(&mut self, id: EntityId) -> Option<&mut T> {
        self.sparse.get_mut(id)
    }

    pub(crate) fn add(&mut self, entity_id: EntityId, component: T) {
        self.sparse.insert(entity_id, component);
    }

    pub(crate) fn clear(&mut self) {
        self.sparse.clear();
    }

    pub(crate) fn add_any(&mut self, entity_id: EntityId, comp: Box<dyn ComponentAny>) {
        // Convert Box<dyn ComponentAny> to Box<dyn Any> using the new method
        let comp_any: Box<dyn Any> = comp.into_any();

        // Downcast the Box<dyn Any> directly to Box<T>
        let comp_t: Box<T> = comp_any.downcast::<T>().expect("Failed to downcast to the specific type");

        // Insert the component into the sparse set, dereferencing the box to get T
        self.sparse.insert(entity_id, *comp_t);
    }

    pub(crate) fn iter(&self) -> SparseArrayIter<EntityId, T> {
        self.sparse.iter()
    }

    pub(crate) fn iter_mut(&mut self) -> SparseArrayIterMut<EntityId, T> {
        self.sparse.iter_mut()
    }

    pub(crate) fn iter_mut_intersection<'a, X: Component>(&'a mut self, other: &'a mut CompStorage<X>) -> SparseArrayIntersectionMutIter<EntityId, T, X> {
        SparseArrayIntersectionMutIter::new(&mut self.sparse, &mut other.sparse)
    }

    pub(crate) fn iter_intersection<'a, X: Component>(&'a self, other: &'a CompStorage<X>) -> SparseArrayIntersectionIter<EntityId, T, X> {
        SparseArrayIntersectionIter::new(&self.sparse, &other.sparse)
    }
}

impl<T: Component + 'static> CompStorageAny for CompStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove(&mut self, entity_id: EntityId) {
        self.sparse.remove(entity_id)
    }
}

