use crate::{Task};
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

        let (components, entities, res) = world.managers();

        for task in &self.schedule {
            task.task().run(components, entities, res);
        }

        world.update()
    }

    fn sort_tasks(&mut self) {
        self.is_sorted = true;
        for i in 0..self.schedule.len() {
            let depend_index = self.schedule[i].find_dependencies(&self.schedule[..i]);
            self.schedule[i].set_depends(depend_index);
        }
    }
}