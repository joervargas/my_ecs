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
    pub id: usize,
    pub generation: u64,
    pub type_ids: HashSet<TypeId>,
    registry: &'a mut Registry
}

impl<'a> EntityBuilder<'a>
{
    pub fn new(id: usize, generation: u64, registry: &'a mut Registry) -> Self
    {
        Self { id, generation, type_ids: HashSet::new(), registry }
    }

    pub fn with_component<T: Any>(&mut self, data: T) -> &mut Self
    {
        let type_id = TypeId::of::<T>();
        if !self.registry.components.contains_key(&type_id)
        {
            self.registry.register_component::<T>();
        }

        if let Some(comps) = self.registry.components.get_mut(&type_id)
        {
            if let Some(vstore) = comps.as_any_mut().downcast_mut::<VecStore<T>>()
            {
                vstore.set(self.id, Some(data));
            }
        }

        self.type_ids.insert(type_id);

        self
    }

    pub fn build(&mut self) -> Entity
    {
        Entity{ id: self.id, generation: self.generation, type_ids: self.type_ids.clone() }
    }

}