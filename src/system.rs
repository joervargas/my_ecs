use std::{
    any::{Any, TypeId}, 
    collections::HashMap, ops::Deref
};

use super::world::World;

pub trait System
{
    fn run(&mut self, world: &mut World);
}

pub struct Dispatch
{
    systems: HashMap<TypeId, Box<dyn System>>
}

impl Dispatch
{
    pub fn new() -> Self
    {
        Self {
            systems: HashMap::new()
        }
    }

    pub fn add_system(&mut self, system: Box<dyn System>)
    {
        let type_id = system.deref().type_id();
        self.systems.insert(type_id, system);
    }

    pub fn remove_system<T: Any>(&mut self)
    {
        let type_id = TypeId::of::<T>();
        self.systems.remove(&type_id);
    }

    pub fn dispatch_systems(&mut self, world: &mut World)
    {
        for system in self.systems.values_mut()
        {
            system.run(world);
        }
    }
}