use crate::component::component::Component;

pub struct CompRead<T: Component> {
    components: Vec<T>,
}