use std::{
    any::{Any, TypeId}, 
    collections::HashMap
};


pub struct Resources
{
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources
{
    pub fn new() -> Self
    {
        Self{
            data: HashMap::new()
        }
    }

    pub fn add(&mut self, data: impl Any)
    {
        let type_id = data.type_id();
        self.data.insert(type_id, Box::new(data));
    }

    pub fn get<T: Any>(&self) -> Option<&T>
    {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get(&type_id)
        {
            data.downcast_ref::<T>()
        } else {
            None
        }
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T>
    {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get_mut(&type_id)
        {
            data.downcast_mut::<T>()
        } else {
            None
        }
    }

    pub fn remove<T: Any>(&mut self)
    {
        let type_id = TypeId::of::<T>();
        self.data.remove(&type_id);
    }
    
}