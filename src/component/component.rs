use std::any::Any;
use cow_macros::Component;

pub trait ComponentAny {
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait Component: ComponentAny + Send + Sync {}
