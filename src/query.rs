use std::{any::{Any, TypeId}, collections::HashSet};

use crate::{component_store::VecStore, registry::Registry};


pub struct QueryBuilder<'a>
{
    // stores: Vec<&'a dyn Any>,
    types: HashSet<TypeId>,
    registry: &'a Registry
}

impl<'a> QueryBuilder<'a>
{
    pub fn new(registry: &'a Registry) -> Self
    {
        Self { types: HashSet::new(), registry }
    }

    pub fn with_component<T: Any>(&mut self) -> &mut Self
    {
        let type_id = TypeId::of::<T>();

        // if component exists in registry
        if self.registry.components.contains_key(&type_id)
        {
            self.types.insert(type_id);
        }
        // if let Some(comps) = self.registry.components.get(&type_id)
        // {
        //     // if let Some(vstore) = comps.as_any().downcast_ref::<VecStore<T>>()
        //     // {
        //     //     self.stores.push(vstore);
        //     // }
        // }
        self
    }

    pub fn get(&self) -> Vec<usize>
    {
        self.registry.get_entity_ids(&self.types)
    }

}