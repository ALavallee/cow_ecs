#[allow(unused)]
#[allow(dead_code)]
pub mod scheduler;
pub mod world;
pub mod schedule;
pub mod component;
pub mod entity;
pub mod resource;
pub mod comps;

pub use cow_macros;
use crate::archetype::archetype_manager::ArchetypeManager;
use crate::commands::EntityCommands;
use crate::resource::res_manager::ResManager;

pub mod commands;
pub mod archetype;

use crate::schedule::task_type::TaskType;


pub trait Task {
    fn name(&self) -> String;

    fn arguments(&self) -> Vec<TaskType>;

    fn run(&self, comps: &mut ArchetypeManager,
           commands: &mut EntityCommands,
           res: &ResManager);
}
