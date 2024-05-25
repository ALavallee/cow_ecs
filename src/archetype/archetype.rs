use std::any::{TypeId};
use std::collections::HashMap;
use crate::component::comp_storage::{CompStorage, CompStorageAny};
use crate::component::component::Component;
use crate::entity::entity::EntityId;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ArchetypeIndex {
    types: Vec<TypeId>,
}

impl ArchetypeIndex {
    pub fn new() -> Self {
        Self { types: vec![] }
    }

    pub fn from_slice(comp_types: &[TypeId]) -> Self {
        Self { types: comp_types.to_vec() }
    }

    pub fn add<T: Component + 'static>(&mut self) -> bool {
        let type_id = TypeId::of::<T>();
        // returns true if it added something
        return match self.types.binary_search(&type_id) {
            Ok(_) => { false } // don't add it if it's already in there
            Err(index) => {
                self.types.insert(index, type_id);
                true
            }
        };
    }

    pub fn contains<T: Component + 'static>(&self) -> bool {
        self.types.contains(&TypeId::of::<T>())
    }

    pub fn remove<T: Component + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.types.retain(|&x| x != type_id);
    }

    pub fn types(&self) -> &Vec<TypeId> {
        &self.types
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }
}


pub struct Archetype {
    index: ArchetypeIndex,
    // entity to archetype id
    entities: HashMap<EntityId, usize>,
    indices: Vec<EntityId>,
    components: Vec<Box<dyn CompStorageAny>>,
}

impl Archetype {
    pub fn new(index: ArchetypeIndex) -> Self {
        Self { index, entities: HashMap::new(), indices: vec![], components: vec![] }
    }

    pub fn index(&self) -> &ArchetypeIndex {
        &self.index
    }

    pub fn insert_comp_storage<T: Component + 'static>(&mut self, comp: T) {
        let mut storage = CompStorage::<T>::new();
        storage.add(comp);
        self.components.push(Box::new(storage))
    }

    pub fn remove_comp_storage<T: Component + 'static>(&mut self) {
        for (i, comp) in self.components.iter().enumerate() {
            if comp.contained_type() == TypeId::of::<T>() {
                self.components.remove(i);
                return;
            }
        }
    }

    pub fn add_without_comp(&mut self, entity_id: EntityId) {
        self.entities.insert(entity_id, self.indices.len());
        self.indices.push(entity_id);
    }

    pub fn add<T: Component + 'static>(&mut self, comp: T) {
        let type_id = TypeId::of::<T>();

        for storage in self.components.iter_mut() {
            if storage.as_ref().contained_type() == type_id {
                if let Some(storage_casted) = storage.as_any_mut().downcast_mut::<CompStorage<T>>() {
                    storage_casted.add(comp);
                    return;
                }
            }
        }
    }

    pub fn update<T: Component + 'static>(&mut self, entity_id: EntityId, comp: T) {
        let type_id = TypeId::of::<T>();
        for storage in self.components.iter_mut() {
            if storage.as_ref().contained_type() == type_id {
                if let Some(storage_casted) = storage.as_any_mut().downcast_mut::<CompStorage<T>>() {
                    storage_casted.update(self.entities[&entity_id], comp);
                    return;
                }
            }
        }
    }

    pub fn remove(&mut self, entity_id: EntityId) {
        // To avoid a potentially costly memory copy, we swap the last element and the element to delete
        // and then do a simple efficient pop
        let index_to_switch = self.entities[&entity_id];
        let last_entity = self.indices.last().unwrap();
        self.entities.insert(*last_entity, index_to_switch);
        self.entities.remove(&entity_id);
        let last_index = self.indices.len() - 1;
        self.indices.swap(index_to_switch, last_index);
        self.indices.pop();
    }

    pub fn query<T: Component + 'static>(&self, entity_id: EntityId) -> Option<&T> {
        if let Some(entity_index) = self.entities.get(&entity_id) {
            for storage in &self.components {
                if let Some(comp_cast) = storage.as_any().downcast_ref::<CompStorage<T>>() {
                    return Some(comp_cast.get(*entity_index));
                }
            }
        }
        None
    }

    pub fn duplicate(&self, new_index: ArchetypeIndex) -> Archetype {
        let mut new_arch = Self::new(new_index);
        for storage in &self.components {
            new_arch.components.push(storage.duplicate());
        }
        new_arch
    }

    pub fn transfer(&mut self, other: &mut Self, entity_id: EntityId) {
        other.entities.insert(entity_id, other.indices.len());
        other.indices.push(entity_id);
        if let Some(entity_index) = self.entities.get(&entity_id) {
            for left_comp in self.components.iter_mut() {
                for right_comp in other.components.iter_mut() {
                    if left_comp.contained_type() == right_comp.contained_type() {
                        left_comp.transfer(right_comp, *entity_index);
                    }
                }
            }
        }
    }

    pub fn indices(&self) -> &Vec<EntityId> {
        &self.indices
    }

    pub fn storage<T: Component + 'static>(&self) -> Option<&Vec<T>> {
        let type_id = TypeId::of::<T>();
        for component in &self.components {
            if component.contained_type() == type_id {
                if let Some(casted_comp) = component.as_any().downcast_ref::<CompStorage<T>>() {
                    return Some(casted_comp.components());
                }
            }
        }

        None
    }

    pub fn storage_mut<T: Component + 'static>(&mut self) -> Option<&mut Vec<T>> {
        let type_id = TypeId::of::<T>();
        for component in self.components.iter_mut() {
            if component.contained_type() == type_id {
                if let Some(casted_comp) = component.as_any_mut().downcast_mut::<CompStorage<T>>() {
                    return Some(casted_comp.components_mut());
                }
            }
        }

        None
    }
}