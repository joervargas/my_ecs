
use std::any::Any;

use crate::resource::Resources;
use crate::registry::Registry;

pub struct World
{
    resources: Resources,
    registry: Registry
}


impl World
{
    pub fn new() -> Self
    {
        Self { 
            resources: Resources::new(),
            registry: Registry::new()
        }
    }

    pub fn register_component<T: Any>(&mut self)
    {}
}