use std::{
    any::{Any, TypeId}, 
    collections::HashSet
};

use crate::{
    component_store::VecStore, 
    entity::Entity, 
    registry::Registry
};


pub struct EntityBuilder<'a>
{
    id: usize,
    pub type_ids: HashSet<TypeId>,
    // pub id: usize,
    // pub generation: u64,
    // pub is_active: bool
    registry: &'a mut Registry,
}

impl<'a> EntityBuilder<'a>
{
    // /// Creates a new EntityBuilder
    // pub fn new(id: usize, generation: u64, is_active: bool, registry: &'a mut Registry) -> Self
    // {
    //     Self { id, generation, type_ids: HashSet::new(), is_active,registry }
    // }

    /// Creates a new EntityBuilder
    pub fn new(registry: &'a mut Registry) -> Self
    {
        let id = registry.entities.activate();

        // if deactiveated entities exist to use, use that
        if !registry.entities.is_deactivated_empty()
        {
            // if let Some(id) = self.generator.deactive_last()
            // {
                registry.components.iter_mut().for_each(
                    | (_type_id, comps) |
                    {
                        comps.set_none(id);
                    }
                );
            // }
        } 
        else // allocate entity data with push to end
        { 
            registry.components.iter_mut().for_each(
                | (_type_id, comps) |
                {
                    comps.push_none();
                }
            );
        }

        Self { id, type_ids: HashSet::new(), registry }
    }

    /// Adds a component to the entity
    pub fn with_component<T: Any>(&mut self, data: T) -> &mut Self
    {
        let type_id = TypeId::of::<T>();
        // if component doesn't exist in registry, add it
        if !self.registry.components.contains_key(&type_id)
        {
            self.registry.register_component::<T>();
        }

        if let Some(comps) = self.registry.components.get_mut(&type_id)
        {
            // downcast Any to VecStore<T>
            if let Some(vstore) = comps.as_any_mut().downcast_mut::<VecStore<T>>()
            {
                // vstore.set(self.id, Some(data));
                if let Some(mut component) = vstore.get_mut(self.id)
                {
                    *component = Some(data);
                }
            }
        }

        // add type_id to type_ids
        self.type_ids.insert(type_id);

        self
    }

    /// Builds the Entity, stores it, and returns the id
    pub fn build(&mut self) -> usize
    {
        // let entity = Entity{ id: self.id, generation: self.generation, type_ids: self.type_ids.clone(), is_active: self.is_active };
        let entity = self.registry.entities.get_mut(self.id).unwrap();
        entity.type_ids = self.type_ids.clone();
        self.id
    }

}