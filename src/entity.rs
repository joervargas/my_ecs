use std::{any::TypeId, collections::HashSet};


// #[derive(Clone, PartialEq, Debug)]
// pub struct Entity
// {
//     pub id: usize, // Id correlates to index position in EntityManager.active_entities
//     pub generation: u64, // Correlates to how many times the EntityManager has reused this ID
//     pub type_ids: HashSet<TypeId>
// }

// pub struct EntityStatus
// {
//     pub is_active: bool, // Flag to mark use
//     pub generation: u64, // Correlates to how many times the EntityManager has reused the corresponding Entity
// }


// pub struct EntityManager
// {
//     active_entities: Vec<EntityStatus>,
//     dropped_ids: Vec<usize> 
// }

// impl EntityManager
// {
//     pub fn new() -> Self
//     {
//         Self {
//             active_entities: Vec::new(),
//             dropped_ids: Vec::new()
//         }
//     }

//     pub fn activate(&mut self) -> Entity
//     {
//         // Previously used entity ID available
//         if let Some(id) = self.dropped_ids.pop()
//         {
//             let entity_status = &mut self.active_entities[id];
//             entity_status.is_active = true;
//             entity_status.generation += 1;
//             return Entity { id, generation: entity_status.generation, type_ids: HashSet::new() };
//         }

//         // No previously used entity IDs available
//         self.active_entities.push(EntityStatus{ is_active: true, generation: 0 });
//         Entity { id: self.active_entities.len() - 1, generation: 0, type_ids: HashSet::new() }
//     }

//     pub fn drop(&mut self, entity: Entity)
//     {
//         // If entity exists in active entities
//         if let Some(entity_status) = self.active_entities.get_mut(entity.id)
//         {
//             if entity_status.is_active && entity_status.generation == entity.generation
//             {
//                 entity_status.is_active = false;
//                 self.dropped_ids.push(entity.id);
//             }
//         }
//     }

//     pub fn activated_size(&self) -> usize
//     {
//         self.active_entities.len()
//     }

//     pub fn is_activated_empty(&self) -> bool
//     {
//         self.active_entities.is_empty()
//     }

//     pub fn deactivated_size(&self) -> usize
//     {
//         self.dropped_ids.len()
//     }

//     pub fn is_deactivated_empty(&self) -> bool
//     {
//         self.dropped_ids.is_empty()
//     }

//     pub fn deactive_last(&self) -> Option<usize>
//     {
//         self.dropped_ids.last().copied()
//     }

// }

// TODO: Add parent / child Entity abiility
#[derive(Clone, PartialEq, Debug)]
pub struct Entity
{
    pub type_ids: HashSet<TypeId>,
    pub id: usize, // Id correlates to index position in EntityManager.active_entities
    // generation maybe not needed, commented out below
    // pub generation: u64, // Correlates to how many times the EntityManager has reused this ID
    pub is_active: bool // Flag to mark use
}

pub struct EntityManager
{
    pub(crate) active: Vec<Entity>,
    pub(crate) dropped: Vec<usize> 
}

impl EntityManager
{
    /// Creates a new EntityManager
    pub fn new() -> Self
    {
        Self {
            active: Vec::new(),
            dropped: Vec::new()
        }
    }

    /// Activates a new entity or reuses an old deactivated one, returns id
    pub fn activate(&mut self) -> usize
    {
        // Previously used entity ID available
        if let Some(id) = self.dropped.pop()
        {
            let entity = &mut self.active[id];
            entity.id = id;
            entity.is_active = true;
            // entity.generation += 1; // add to the generation
            entity.type_ids = HashSet::new();
            return id;
        }

        // No previously used entity IDs available
        // first generation = 0
        let entity = Entity { id: self.active.len(), type_ids: HashSet::new(), is_active: true };
        let id = entity.id;
        self.active.push(entity);
        id
    }

    // /// Drops (or deactivates) an entity
    // pub fn drop(&mut self, entity: Entity)
    // {
    //     // If entity exists in active entities
    //     if let Some(e) = self.active.get_mut(entity.id)
    //     {
    //         // If entity exists in active entities and is the same generation
    //         if e.is_active && e.generation == entity.generation
    //         {
    //             e.is_active = false;
    //             self.dropped.push(e.id);
    //         }
    //     }
    // }

    /// Drops (or deactivates) an entity
    pub fn drop(&mut self, id: usize)
    {
        // If entity exists in active entities
        if let Some(e) = self.active.get_mut(id)
        {
            // If entity exists in active entities
            if e.is_active 
            {
                e.is_active = false;
                self.dropped.push(e.id);
            }
        }
    }

    /// Returns the number of active entities
    pub fn activated_size(&self) -> usize
    {
        self.active.len()
    }

    /// Returns true if there are no active entities
    pub fn is_activated_empty(&self) -> bool
    {
        self.active.is_empty()
    }

    /// Returns the number of deactivated entities
    pub fn deactivated_size(&self) -> usize
    {
        self.dropped.len()
    }

    /// Returns true if there are no deactivated entities
    pub fn is_deactivated_empty(&self) -> bool
    {
        self.dropped.is_empty()
    }

    /// Returns the id of the last deactivated entity
    pub fn deactive_last(&self) -> Option<usize>
    {
        self.dropped.last().copied()
    }

    /// Returns an entity given an id
    pub fn get(&self, id: usize) -> Option<&Entity>
    {
        self.active.get(id)
    }

    /// Returns a mutable entity given an id
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Entity>
    {
        self.active.get_mut(id)
    }

    /// Returns a vector of entity ids given a set of component TypeIds
    pub fn filter_by_components(&self, type_ids: &HashSet<TypeId>) -> Vec<usize>
    {
        let mut ids: Vec<usize> = Vec::new();
        // Iterate over active entities
        for e in &self.active
        {
            // If entity has all components
            if type_ids.is_subset(&e.type_ids)
            {
                // Add entity id to ids vec
                ids.push(e.id);
            }
        }

        // return ids
        ids
    }

}


#[cfg(test)]
mod tests{
    
    use super::*;

    #[test]
    fn test_drop()
    {
        let mut entities = EntityManager::new();

        let e1 = entities.activate();
        assert_eq!(e1, 0);

        let e2 = entities.activate();
        entities.activate();
        entities.activate();
        // entities.drop(entities.get(e2).unwrap());
        entities.drop(e2);
        let e3 = entities.activate();
        let entity3 = entities.get(e3).unwrap();

        assert_eq!(*entity3, Entity{id: 1, type_ids: HashSet::new(), is_active: true});
    }
}