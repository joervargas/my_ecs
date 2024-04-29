use std::{
    any::{Any, TypeId}, 
    collections::HashMap
};

use crate::{
    component_store::{ComponentStore, VecStore}, 
    entity::IdGenerationManager, 
    entity_builder::EntityBuilder, 
    query::QueryBuilder
};

pub struct Registry
{
    pub components: HashMap<TypeId, Box<dyn ComponentStore>>,
    generator: IdGenerationManager
}

impl Registry
{
    pub fn new() -> Self
    {
        Self {
            components: HashMap::new(),
            generator: IdGenerationManager::new()
        }
    }

    pub fn register_component<T: Any>(&mut self)
    {
        let type_id = TypeId::of::<T>();
        let mut comps: VecStore<T> = VecStore::new();
        // if entities already exist, populate the Vec with None
        if !self.generator.is_activated_empty()
        {
            comps.resize_to_nones(self.generator.activated_size());
        }

        self.components.insert(type_id, Box::new(comps));
    }

    // pub fn create_entity(&mut self) -> &mut Self
    pub fn create_entity(&mut self) -> EntityBuilder
    {
        let entity = self.generator.activate();
        // if deactiveated entities exist to use, use that
        if !self.generator.is_deactivated_empty()
        {
            // if let Some(id) = self.generator.deactive_last()
            // {
                self.components.iter_mut().for_each(
                    | (_type_id, comps) |
                    {
                        comps.set_none(entity.id);
                    }
                );
            // }
        } 
        else // allocate entity data with push to end
        { 
            self.components.iter_mut().for_each(
                | (_type_id, comps) |
                {
                    comps.push_none();
                }
            );
        }

        EntityBuilder::new(entity.id, entity.generation, self)
    }

    pub fn get_components<T: Any>(&self) -> Option<&VecStore<T>>
    {
        let type_id = TypeId::of::<T>();
        if let Some(comps) = self.components.get(&type_id)
        {
            return comps.as_any().downcast_ref::<VecStore<T>>();
        } else {
            return None;
        }
    }

    pub fn get_components_mut<T: Any>(&mut self) -> Option<&mut VecStore<T>>
    {
        let type_id = TypeId::of::<T>();
        if let Some(comps) = self.components.get_mut(&type_id)
        {
            return comps.as_any_mut().downcast_mut::<VecStore<T>>();
        } else {
            return None;
        }
    }

    pub fn create_query(&self) -> QueryBuilder
    {
        todo!()
    }

}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn create_entity()
    {
        let mut registry = Registry::new();
        registry.register_component::<Health>();
        registry.register_component::<Speed>();

        let entity = registry.create_entity()
            .with_component::<Health>(Health{value: 100})
            .with_component::<Speed>(Speed{value: 100})
            .build();

        assert_eq!(entity.id, 0);
        assert_eq!(entity.type_ids.contains(&TypeId::of::<Health>()), true);
        assert_eq!(entity.type_ids.contains(&TypeId::of::<Speed>()), true);

        assert_eq!(registry.components.contains_key(&TypeId::of::<Health>()), true);
    }

    struct Health
    {
        pub value: u32
    }

    struct Speed
    {
        pub value: u32
    }
}