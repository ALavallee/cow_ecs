use crate::component::component::Component;

pub struct CompWrite<T: Component> {
    components: Vec<T>,
}