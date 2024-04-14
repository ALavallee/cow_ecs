use crate::component::comp_manager::CompManager;
use crate::component::component::Component;
use crate::entity::entity::EntityId;
use crate::entity::entity_lock::EntityLock;
use crate::resource::res_manager::ResManager;
use crate::resource::resource::Resource;

pub struct World {
    resources: ResManager,
    components: CompManager,
    entities: EntityLock,
}

impl World {
    pub fn new() -> Self {
        Self { components: CompManager::new(), entities: EntityLock::new(), resources: ResManager::new() }
    }

    pub fn create(&mut self) -> EntityId {
        self.entities.create()
    }

    pub fn release(&mut self, entity_id: EntityId) {
        self.entities.release(entity_id);
        self.components.remove(entity_id);
    }

    pub fn add<T: Component + 'static>(&mut self, entity_id: EntityId, comp: T) {
        self.components.register::<T>();
        if let Some(comps) = self.components.query_mut::<T>() {
            comps.add(entity_id, comp);
        }
    }

    pub fn remove<T: Component + 'static>(&mut self, entity_id: EntityId) {
        self.components.register::<T>();
        if let Some(comps) = self.components.query_mut::<T>() {
            comps.remove(entity_id);
        }
    }

    pub fn set_res<T: Resource + 'static>(&mut self, res: T) {
        self.resources.set(res)
    }

    pub fn entities_count(&self) -> usize {
        self.entities.count()
    }

    pub fn components(&self) -> &CompManager {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut CompManager {
        &mut self.components
    }

    pub fn managers(&mut self) -> (&mut CompManager, &mut EntityLock, &mut ResManager) {
        (&mut self.components, &mut self.entities, &mut self.resources)
    }
}