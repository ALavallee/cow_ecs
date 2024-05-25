use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use crate::archetype::archetype::{Archetype, ArchetypeIndex};
use crate::archetype::archetype_query::{ArchetypeQuery, ArchetypeQueryMut};
use crate::component::component::{Component};
use crate::entity::entity::EntityId;

pub struct ArchetypeManager {
    // link  current archetype of an entity
    entities: HashMap<EntityId, usize>,
    // link an index with an archetype
    archetypes_types: HashMap<ArchetypeIndex, usize>,
    // link all archetypes where are contained type is contained where
    archetypes_contains: HashMap<TypeId, HashSet<usize>>,
    // the archetypes
    archetypes: Vec<Archetype>,
}

impl ArchetypeManager {
    pub fn new() -> Self {
        // create the basic archetype empty
        let mut archetypes_types = HashMap::new();
        archetypes_types.insert(ArchetypeIndex::new(), 0);

        Self {
            entities: HashMap::new(),
            archetypes_types,
            archetypes_contains: HashMap::new(),
            archetypes: vec![Archetype::new(ArchetypeIndex::new())],
        }
    }

    pub fn add_entity(&mut self, entity_id: EntityId) {
        self.entities.insert(entity_id, 0);
        self.archetypes[0].add_without_comp(entity_id);
    }

    pub fn add<T: Component + 'static>(&mut self, entity_id: EntityId, comp: T) {
        // entity exist
        let arch_before_add = &self.archetypes[self.entities[&entity_id]].index().clone();

        let mut new_arch = arch_before_add.clone();
        let is_update = !new_arch.add::<T>();

        // is the archetype of the entity changed (could be a simple update) ?
        // if so we need to transfer it of archetype
        if !is_update {

            // does the new archetype exist
            let new_arch_id = if let Some(entity_arch_types) = self.archetypes_types.get(&new_arch) {
                if let Some(old_archetype_index) = self.archetypes_types.get(arch_before_add) {
                    if old_archetype_index != entity_arch_types {

                        // Ensure that we have two distinct indices
                        let (old_archetype, new_archetype) = if old_archetype_index < entity_arch_types {
                            let (first_part, second_part) = self.archetypes.split_at_mut(*entity_arch_types);
                            let old_archetype = &mut first_part[*old_archetype_index];
                            let archetype = &mut second_part[0];
                            (old_archetype, archetype)
                        } else {
                            let (first_part, second_part) = self.archetypes.split_at_mut(*old_archetype_index);
                            let archetype = &mut first_part[*entity_arch_types];
                            let old_archetype = &mut second_part[0];
                            (old_archetype, archetype)
                        };

                        old_archetype.transfer(new_archetype, entity_id);
                        new_archetype.add(comp);
                    }
                }
                *entity_arch_types
            } else {

                //archetype doesn't exist, create it
                // replicate the old archetype
                let new_arch_id = self.archetypes_types.len();
                let old_arch = self.archetypes_types[arch_before_add];
                let mut new_archetype = self.archetypes[old_arch].duplicate(new_arch.clone());
                new_archetype.insert_comp_storage::<T>(comp);

                if let Some(old_archetype_index) = self.archetypes_types.get(arch_before_add) {
                    self.archetypes[*old_archetype_index].transfer(&mut new_archetype, entity_id);
                }

                self.archetypes.push(new_archetype);
                self.set_all_contained(&new_arch, new_arch_id);
                self.archetypes_types.insert(new_arch, new_arch_id);
                new_arch_id
            };

            // remove it from the old archetype
            self.archetypes[self.entities[&entity_id]].remove(entity_id);

            // update the archetypes of the entity
            self.entities.insert(entity_id, new_arch_id);
        } else {
            self.archetypes[self.entities[&entity_id]].update(entity_id, comp);
        }
    }

    pub fn remove<T: Component + 'static>(&mut self, entity_id: EntityId) {
        let old_archetype_index = self.entities[&entity_id];
        if let Some(old_archetype) = self.archetypes.get(old_archetype_index) {
            let mut new_arch_types = old_archetype.index().clone();
            new_arch_types.remove::<T>();
            // does the current arch contain the component
            let arch_target_index = if let Some(entity_arch_types) = self.archetypes_types.get(&new_arch_types) {
                *entity_arch_types
            } else {
                self.archetypes_types.insert(new_arch_types.clone(), self.archetypes.len());
                let mut new_arch = old_archetype.duplicate(new_arch_types.clone());
                new_arch.remove_comp_storage::<T>();
                self.archetypes.push(new_arch);
                let new_arch_id = self.archetypes.len() - 1;
                self.set_all_contained(&new_arch_types, new_arch_id);
                new_arch_id
            };

            if old_archetype_index != arch_target_index {

                // Ensure that we have two distinct indices
                let (old_archetype, new_archetype) = if old_archetype_index < arch_target_index {
                    let (first_part, second_part) = self.archetypes.split_at_mut(arch_target_index);
                    let old_archetype = &mut first_part[old_archetype_index];
                    let archetype = &mut second_part[0];
                    (old_archetype, archetype)
                } else {
                    let (first_part, second_part) = self.archetypes.split_at_mut(old_archetype_index);
                    let archetype = &mut first_part[arch_target_index];
                    let old_archetype = &mut second_part[0];
                    (old_archetype, archetype)
                };

                // Get mutable references to archetypes at min_index and max_index
                old_archetype.transfer(new_archetype, entity_id);
                old_archetype.remove(entity_id);

                self.entities.insert(entity_id, arch_target_index);
            }
        }
    }

    pub fn query<T: Component + 'static>(&self, entity_id: EntityId) -> Option<&T> {
        if let Some(arch_id) = self.entities.get(&entity_id) {
            return self.archetypes[*arch_id].query::<T>(entity_id);
        }
        None
    }

    pub fn fetch_info<T: Component>(&self) -> ArchetypeQuery<T> {
        let type_id = TypeId::of::<T>();
        let mut storages = Vec::new();
        let mut indices = Vec::new();
        if let Some(index_for_storage) = self.archetypes_contains.get(&type_id) {
            for index in index_for_storage {
                if let Some(storage) = self.archetypes[*index].storage::<T>() {
                    indices.push(self.archetypes[*index].indices());
                    storages.push(storage);
                }
            }
        }

        ArchetypeQuery::new(indices, storages)
    }

    pub fn fetch_info_mut<T: Component>(&mut self) -> ArchetypeQueryMut<T> {
        let type_id = TypeId::of::<T>();
        let mut storages = Vec::new();
        let mut indices = Vec::new();

        // Get the raw pointer to the archetypes array.
        let archetypes_ptr = self.archetypes.as_mut_ptr();

        if let Some(index_for_storage) = self.archetypes_contains.get(&type_id) {
            // Reserve space to avoid reallocations that could invalidate pointers.
            storages.reserve(index_for_storage.len());
            indices.reserve(index_for_storage.len());

            for &index in index_for_storage.iter() {
                unsafe {
                    // Access each archetype by index using the raw pointer.
                    let archetype = &mut *archetypes_ptr.add(index);
                    let storage_ptr = archetype.storage_mut::<T>();

                    if let Some(storage) = storage_ptr {
                        // Push the mutable reference converted from a raw pointer.
                        storages.push(&mut *(storage as *mut _));

                        // Directly access indices function and convert to raw pointer and back to ref.
                        let indices_ptr = archetype.indices() as *const _;
                        indices.push(&*indices_ptr);
                    }
                }
            }
        }

        ArchetypeQueryMut::new(indices, storages)
    }

    fn set_all_contained(&mut self, archetype_index: &ArchetypeIndex, new_index: usize) {
        for type_index in archetype_index.types() {
            if self.archetypes_contains.get(&type_index).is_none() {
                self.archetypes_contains.insert(*type_index, HashSet::new());
            }

            if let Some(indices) = self.archetypes_contains.get_mut(&type_index) {
                indices.insert(new_index);
            }
        }
    }
}