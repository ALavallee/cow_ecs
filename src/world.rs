use crate::archetype::archetype_manager::ArchetypeManager;
use crate::component::component::Component;
use crate::entity::entity::EntityId;
use crate::entity::entity_manager::EntityManager;
use crate::resource::res_manager::ResManager;
use crate::resource::resource::Resource;

pub struct World {
    resources: ResManager,
    archetypes: ArchetypeManager,
    entities: EntityManager,
}

impl World {
    pub fn new() -> Self {
        Self { archetypes: ArchetypeManager::new(), entities: EntityManager::new(), resources: ResManager::new() }
    }

    pub fn create(&mut self) -> EntityId {
        let entity_id = self.entities.create();
        self.archetypes.add_entity(entity_id);
        entity_id
    }

    pub fn release(&mut self, entity_id: EntityId) {
        self.entities.release(entity_id);
    }

    pub fn add<T: Component + 'static>(&mut self, entity_id: EntityId, comp: T) {
        self.archetypes.add(entity_id, comp);
    }

    pub fn remove<T: Component + 'static>(&mut self, entity_id: EntityId) {
        self.archetypes.remove::<T>(entity_id);
    }

    pub fn query<T: Component + 'static>(&self, entity_id: EntityId) -> Option<&T> {
        self.archetypes.query::<T>(entity_id)
    }

    pub fn set_res<T: Resource + 'static>(&mut self, res: T) {
        self.resources.set(res)
    }

    pub fn entities_count(&self) -> usize {
        self.entities.count()
    }

    pub fn managers(&mut self) -> (&mut ArchetypeManager, &mut ResManager) {
        (&mut self.archetypes, &mut self.resources)
    }
}