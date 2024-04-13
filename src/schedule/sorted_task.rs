use crate::Task;

pub struct SortedTask {
    task: Box<dyn Task>,
    depends: Option<usize>,
}

impl SortedTask {
    pub fn new(task: Box<dyn Task>) -> Self {
        Self { task, depends: None }
    }

    pub fn set_depends(&mut self, value: Option<usize>) {
        self.depends = value
    }

    pub fn find_dependencies(&self, tasks: &[SortedTask]) -> Option<usize> {
        for i in (0..tasks.len()).rev() {
            if self.check_if_depends(&tasks[i]) {
                return Some(i);
            }
        }

        None
    }

    pub fn check_if_depends(&self, other: &Self) -> bool {
        let self_args = self.task.arguments();
        let other_args = other.task.arguments();
        for self_arg in self_args {
            for other_arg in &other_args {
                if self_arg.is_dependant(other_arg) {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn depends_on(&self) -> Option<usize> {
        self.depends
    }

    pub fn task(&self) -> &Box<dyn Task> {
        &self.task
    }
}