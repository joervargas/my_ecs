use std::any::{Any, TypeId};

use crate::{component_store::VecStore, registry::Registry};



pub struct QueryBuilder<'a>
{
    stores: Vec<&'a dyn Any>,
    registry: &'a Registry
}

impl<'a> QueryBuilder<'a>
{
    pub fn new(registry: &'a Registry) -> Self
    {
        Self { stores: Vec::new(), registry }
    }

    pub fn with_component<T: Any>(&mut self) -> &mut Self
    {
        let type_id = TypeId::of::<T>();

        if let Some(comps) = self.registry.components.get(&type_id)
        {
            if let Some(vstore) = comps.as_any().downcast_ref::<VecStore<T>>()
            {
                self.stores.push(vstore);
            }
        }
        self
    }

}