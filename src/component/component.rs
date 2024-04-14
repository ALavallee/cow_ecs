use std::any::Any;

pub trait ComponentAny {
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait Component: ComponentAny + Send + Sync {}