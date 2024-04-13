#[allow(unused)]
#[allow(dead_code)]
#[allow(unused_attributes)]
pub mod scheduler;
pub mod world;
pub mod schedule;
pub mod component;
pub mod entity;
pub mod resource;
pub mod comps;

pub use cow_macros;

mod data;

use crate::component::comp_manager::CompManager;
use crate::entity::entity_lock::EntityLock;
use crate::resource::res_manager::ResManager;
use crate::schedule::task_type::TaskType;
use crate::world::World;

pub trait Task {
    fn name(&self) -> String;

    fn register(&self, world: &mut World);

    fn arguments(&self) -> Vec<TaskType>;

    fn run(&self, components: &CompManager, entities: &EntityLock, resource: &ResManager);
}
