
use std::any::Any;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use crate::component_store::VecStore;
use crate::entity::Entity;
use crate::entity_builder::EntityBuilder;
use crate::query::QueryBuilder;
use crate::resource::{ResourceError, Resources};
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

    /// Adds a new component to the registry
    pub fn register_component<T: Any + Send + Sync>(&mut self)
    {
        self.registry.register_component::<T>();
    }

    /// Creates a new EntityBuilder instance
    pub fn create_entity(&mut self) -> EntityBuilder
    {
        self.registry.create_entity()
    }

    /// Revtrieves a Vecstore of Type T components if they exist
    pub fn get_components<T: Any>(&self) -> Option<&VecStore<T>>
    {
        self.registry.get_components::<T>()
    }

    /// Revtrieves a mutable Vecstore of Type T components if they exist
    pub fn get_components_mut<T: Any>(&mut self) -> Option<&mut VecStore<T>>
    {
        self.registry.get_components_mut::<T>()
    }

    /// Returns an entity given an id if it exists
    pub fn get_entity(&self, id: usize) -> Option<&Entity>
    {
        self.registry.get_entity(id)
    }

    /// Returns a mutable entity given an id if it exists
    pub fn get_entity_mut(&mut self, id: usize) -> Option<&mut Entity>
    {
        self.registry.get_entity_mut(id)
    }

    /// Starts QueryBuilder
    pub fn query(&self) -> QueryBuilder
    {
        self.registry.query()
    }

    pub fn add_resource<T: Any>(&mut self, resource: T)
    {
        self.resources.add(resource);
    }

    pub fn get_resource<T: Send + Sync + 'static>(&self) -> Result<RwLockReadGuard<T>, ResourceError>
    {
        self.resources.get::<T>()
    }

    pub fn get_resource_mut<T: Send + Sync + 'static>(&mut self) -> Result<RwLockWriteGuard<T>, ResourceError>
    {
        self.resources.get_mut::<T>()
    }

    pub fn remove_resource<T: Any>(&mut self)
    {
        self.resources.remove::<T>();
    }
    
}