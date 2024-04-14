use crate::{Task};
use crate::commands::{EntityCommand, EntityCommands};
use crate::schedule::sorted_task::SortedTask;
use crate::world::World;

pub struct Scheduler {
    schedule: Vec<SortedTask>,
    is_sorted: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { schedule: vec![], is_sorted: false }
    }

    pub fn add_task<T: Task + 'static>(&mut self, new_task: T) {
        self.is_sorted = false;
        self.schedule.push(SortedTask::new(Box::new(new_task)))
    }

    pub fn run(&mut self, world: &mut World) {
        if self.is_sorted == false {
            self.sort_tasks();
        }


        for task in &self.schedule {
            let mut commands = EntityCommands::new();
            {
                let (comps_manager, entities, res) = world.managers();
                task.task().run(comps_manager, &mut commands, res);
            }
            
            for cmd in commands.take_commands().into_iter() {
                match cmd {
                    EntityCommand::NewEntity(components) => {
                        for (comp_type_id, comp) in components {
                            let new_entity = world.create();
                            let (comps_manager, _, _) = world.managers();
                            comps_manager.insert_by_comp_id(comp_type_id, new_entity, comp);
                        }
                    }
                    EntityCommand::RemoveEntity(entity_id) => {
                        world.release(entity_id);
                    }
                }
            }
        }
    }

    fn sort_tasks(&mut self) {
        self.is_sorted = true;
        for i in 0..self.schedule.len() {
            let depend_index = self.schedule[i].find_dependencies(&self.schedule[..i]);
            self.schedule[i].set_depends(depend_index);
        }
    }
}