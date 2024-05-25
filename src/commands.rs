use std::any::TypeId;
use std::collections::HashMap;
use crate::component::component::{Component, ComponentAny};
use crate::entity::entity::EntityId;


pub enum EntityCommand {
    NewEntity(EntityId, HashMap<TypeId, Box<dyn ComponentAny>>),
    ReleaseEntity(EntityId),
}

pub struct EntityCommands {
    commands: Vec<EntityCommand>,
}

impl EntityCommands {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn create(&mut self) -> &mut EntityCommand {
        //self.commands.push(EntityCommand::NewEntity(HashMap::new()));
        self.commands.last_mut().unwrap()
    }

    pub fn release(&mut self, entity_id: EntityId) {
        self.commands.push(EntityCommand::ReleaseEntity(entity_id))
    }
}

impl EntityCommand {
    pub fn add<T: Component + 'static>(&mut self, comp: T) {
        match self {
            EntityCommand::NewEntity(_id, ref mut components) => {
                let type_id = TypeId::of::<T>();
                components.insert(type_id, Box::new(comp));
            }
            EntityCommand::ReleaseEntity(_) => {}
        }
    }
}
