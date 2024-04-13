use std::any::TypeId;

#[derive(Debug)]
pub enum TaskType {
    Comp(TypeId),
    CompMut(TypeId),
    Res(TypeId),
    ResMut(TypeId),
    Entities(),
}

impl TaskType {
    pub fn is_dependant(&self, other: &TaskType) -> bool {
        match self {
            TaskType::Comp(type_id) => {
                match other {
                    // a component can only be read if the component is done being written
                    TaskType::CompMut(other_id) => { return type_id == other_id; }
                    _ => {}
                }
            }
            TaskType::CompMut(type_id) => {
                match other {
                    // a component can only be written if it's done being read or written
                    TaskType::CompMut(other_id) => { return type_id == other_id; }
                    TaskType::Comp(other_id) => { return type_id == other_id; }
                    _ => {}
                }
            }
            TaskType::Res(type_id) => {
                match other {
                    // a res can only be read if the res is done being written
                    TaskType::ResMut(other_id) => { return type_id == other_id; }
                    _ => {}
                }
            }
            TaskType::ResMut(type_id) => {
                match other {
                    // a res can only be written if it's done being read or written
                    TaskType::Res(other_id) => { return type_id == other_id; }
                    TaskType::ResMut(other_id) => { return type_id == other_id; }
                    _ => {}
                }
            }
            TaskType::Entities() => {
                return false;
            }
        }
        false
    }
}
