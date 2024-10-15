use std::{
    any::{Any, TypeId}, 
    collections::{HashMap, HashSet}
};

use std::thread;

use crate::{
    component_store::{ComponentStore, VecStore}, 
    entity::{Entity, EntityManager}, 
    entity_builder::EntityBuilder, query::QueryBuilder, 
    // query::QueryBuilder
};

pub struct Registry
{
    pub components: HashMap<TypeId, Box<dyn ComponentStore>>,
    pub(crate) entities: EntityManager
}

impl Registry
{
    /// Creates a new registry
    pub fn new() -> Self
    {
        Self {
            components: HashMap::new(),
            entities: EntityManager::new()
        }
    }

    /// Adds a new component to the registry
    pub fn register_component<T: Any + Send + Sync>(&mut self)
    {
        let type_id = TypeId::of::<T>();
        let mut comps: VecStore<T> = VecStore::new();
        // if entities already exist, populate the Vec with None
        if !self.entities.is_activated_empty()
        {
            comps.resize_to_nones(self.entities.activated_size());
        }

        // let b_comps: Box<dyn ComponentStore + Send + Sync> = Box::new(comps);
        self.components.insert(type_id, Box::new(comps));
    }

    /// Creates a new EntityBuilder instance
    pub fn create_entity(&mut self) -> EntityBuilder
    {
        EntityBuilder::new(self)
    }

    /// Revtrieves a Vecstore of Type T components if they exist
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

    /// Revtrieves a mutable Vecstore of Type T components if they exist
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

    /// Returns an entity given an id if it exists
    pub fn get_entity(&self, id: usize) -> Option<&Entity>
    {
        self.entities.get(id)
    }

    /// Returns a mutable entity given an id if it exists
    pub fn get_entity_mut(&mut self, id: usize) -> Option<&mut Entity>
    {
        self.entities.get_mut(id)
    }

    /// Returns a vector of entity ids given a set of components TypeIds
    pub fn get_entity_ids(&self, type_ids: &HashSet<TypeId>) -> Vec<usize>
    {
        self.entities.filter_by_components(&type_ids)
    }

    /// Starts QueryBuilder
    pub fn query(&self) -> QueryBuilder
    {
        QueryBuilder::new(self)
    }

}

#[cfg(test)]
mod tests
{
    use crate::{component_store::ComponentError, entity};

    use super::*;

    #[test]
    fn create_entity() -> anyhow::Result<()>
    {
        let mut registry = Registry::new();
        registry.register_component::<Health>();
        registry.register_component::<Speed>();

        let e1 = registry.create_entity()
            .with_component::<Health>(Health{value: 100})
            .with_component::<Speed>(Speed{value: 100})
            .build();

        let entity1 = registry.get_entity(e1).unwrap();
        
        assert_eq!(e1, 0);
        assert_eq!(entity1.type_ids.contains(&TypeId::of::<Health>()), true);
        assert_eq!(entity1.type_ids.contains(&TypeId::of::<Speed>()), true);
        
        assert_eq!(registry.components.contains_key(&TypeId::of::<Health>()), true);

        let e2 = registry.create_entity()
            .with_component::<Health>(Health{value: 100})
            .with_component::<Position>(Position{X: 1.0, Y: 1.0})
            .build();

        let entity2 = registry.get_entity(e2).unwrap();

        assert_eq!(e2, 1);
        assert_eq!(entity2.type_ids.contains(&TypeId::of::<Health>()), true);
        assert_eq!(entity2.type_ids.contains(&TypeId::of::<Position>()), true);

        let query = registry.query()
            .with_component::<Health>()
            // .with_component::<Position>()
            .get();

        assert_eq!(query.len(), 2);
        // assert_eq!(query[0], entity2.id);

        for id in query
        {
            thread::scope(|s|
                {
                    s.spawn(||
                    {
                        let mut health = registry.get_components_mut::<Health>().unwrap().get_mut(id).expect("Failed to get Health component.");
                        health.as_mut().unwrap().value -= 5;
            
                        assert_eq!(health.as_ref().unwrap().value, 95);
                        // Ok(())
                    });
                }
            );
        }
    
        Ok(())
    }

    struct Health
    {
        pub value: u32
    }

    struct Speed
    {
        pub value: u32
    }

    struct Position
    {
        pub X: f32,
        pub Y: f32
    }
}