use std::collections::HashMap;
use crate::{Task};
use crate::commands::{EntityCommand, EntityCommands};
use crate::schedule::sorted_task::SortedTask;
use crate::world::World;

struct SchedulerBlock {
    tasks: Vec<SortedTask>,
}

pub struct Scheduler {
    blocks: HashMap<u32, SchedulerBlock>,
    is_sorted: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { blocks: HashMap::new(), is_sorted: false }
    }

    pub fn add_task<T: Task + 'static>(&mut self, new_task: T) {
        let new_task = SortedTask::new(Box::new(new_task));
        self.is_sorted = false;
        if let Some(block) = self.blocks.get_mut(&0) {
            block.tasks.push(new_task);
        } else {
            let new_block = SchedulerBlock { tasks: vec![new_task] };
            self.blocks.insert(0, new_block);
        }
    }

    pub fn add_task_in_block<T: Task + 'static>(&mut self, block_id: u32, new_task: T) {
        let new_task = SortedTask::new(Box::new(new_task));
        self.is_sorted = false;
        if let Some(block) = self.blocks.get_mut(&block_id) {
            block.tasks.push(new_task);
        } else {
            let new_block = SchedulerBlock { tasks: vec![new_task] };
            self.blocks.insert(block_id, new_block);
        }
    }

    pub fn run(&mut self, world: &mut World) {
        if self.is_sorted == false {
            self.sort_tasks();
        }

        for (_, block) in self.blocks.iter() {
            for task in &block.tasks {
                let mut commands = EntityCommands::new();
                {
                    let (mut archs, res) = world.managers();
                    task.task().run(&mut archs, &mut commands, res);
                }

                /*for cmd in commands.take_commands().into_iter() {
                    match cmd {
                        EntityCommand::NewEntity(components) => {
                            for (comp_type_id, comp) in components {
                                let new_entity = world.create();
                                let (comps_manager, _, _) = world.managers();
                                comps_manager.insert_by_comp_id(comp_type_id, new_entity, comp);
                            }
                        }
                        EntityCommand::ReleaseEntity(entity_id) => {
                            world.release(entity_id);
                        }
                    }
                }*/
            }
        }
    }

    fn sort_tasks(&mut self) {
        self.is_sorted = true;
        for (_, block) in self.blocks.iter_mut() {
            for i in 0..block.tasks.len() {
                let depend_index = block.tasks[i].find_dependencies(&block.tasks[..i]);
                block.tasks[i].set_depends(depend_index);
            }
        }
    }
}