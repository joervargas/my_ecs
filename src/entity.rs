use std::{any::TypeId, collections::HashSet};



#[derive(Clone, PartialEq, Debug)]
pub struct Entity
{
    pub id: usize, // Id correlates to index position in IdGenerationManager.active_entities
    pub generation: u64, // Correlates to how many times the IdGenerationManager has reused this ID
    pub type_ids: HashSet<TypeId>
}

pub struct EntityStatus
{
    pub is_active: bool, // Flag to mark use
    pub generation: u64, // Correlates to how many times the IdGenerationManager has reused the corresponding Entity
}

pub struct IdGenerationManager
{
    active_entities: Vec<EntityStatus>,
    dropped_ids: Vec<usize> 
}

impl IdGenerationManager
{
    pub fn new() -> Self
    {
        Self {
            active_entities: Vec::new(),
            dropped_ids: Vec::new()
        }
    }

    pub fn activate(&mut self) -> Entity
    {
        // Previously used entity ID available
        if let Some(id) = self.dropped_ids.pop()
        {
            let entity_status = &mut self.active_entities[id];
            entity_status.is_active = true;
            entity_status.generation += 1;
            return Entity { id, generation: entity_status.generation, type_ids: HashSet::new() };
        }

        // No previously used entity IDs available
        self.active_entities.push(EntityStatus{ is_active: true, generation: 0 });
        Entity { id: self.active_entities.len() - 1, generation: 0, type_ids: HashSet::new() }
    }

    pub fn drop(&mut self, entity: Entity)
    {
        // If entity exists in active entities
        if let Some(entity_status) = self.active_entities.get_mut(entity.id)
        {
            if entity_status.is_active && entity_status.generation == entity.generation
            {
                entity_status.is_active = false;
                self.dropped_ids.push(entity.id);
            }
        }
    }

    pub fn activated_size(&self) -> usize
    {
        self.active_entities.len()
    }

    pub fn is_activated_empty(&self) -> bool
    {
        self.active_entities.is_empty()
    }

    pub fn deactivated_size(&self) -> usize
    {
        self.dropped_ids.len()
    }

    pub fn is_deactivated_empty(&self) -> bool
    {
        self.dropped_ids.is_empty()
    }

    pub fn deactive_last(&self) -> Option<usize>
    {
        self.dropped_ids.last().copied()
    }

}


#[cfg(test)]
mod tests{
    

    use super::*;

    #[test]
    fn test_drop()
    {
        let mut entity_generator = IdGenerationManager::new();

        let entity = entity_generator.activate();
        assert_eq!(entity, Entity { id: 0, generation: 0, type_ids: HashSet::new() });

        let entity2 = entity_generator.activate();
        entity_generator.activate();
        entity_generator.activate();
        entity_generator.drop(entity2);
        let entity3 = entity_generator.activate();

        assert_eq!(entity3, Entity{id: 1, generation: 1, type_ids: HashSet::new() });
    }
}