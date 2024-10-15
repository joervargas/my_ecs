use std::{
    any::{Any, TypeId}, 
    collections::HashMap, error, fmt, 
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}
};

pub struct ResourceError
{
    // code: usize,
    message: String
}

impl fmt::Display for ResourceError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for ResourceError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "{}", self.message)
    }
}

impl<T: error::Error + Send + Sync + 'static> From<T> for ResourceError 
{
    fn from(e: T) -> Self 
    {
        Self { message: e.to_string()}
    }
}

pub trait ResourceType
{
    type Object: Any + Send + Sync;

    fn get(&self) -> Result<RwLockReadGuard<Self::Object>, ResourceError>;

    fn get_mut(&mut self) -> Result<RwLockWriteGuard<Self::Object>, ResourceError>;
}

pub struct Resource<T>
{
    data: RwLock<T>
}

impl<T> Resource<T>
{
    pub fn new(data: T) -> Self
    {
        Self
        {
            data: RwLock::new(data)
        }
    }
}

impl<T: Send + Sync + 'static> ResourceType for Resource<T>
{
    type Object = T;

    fn get(&self) -> Result<RwLockReadGuard<Self::Object>, ResourceError>
    {
        match self.data.read()
        {
            Ok(data) => Ok(data),
            Err(err) => 
            {
                let type_id = TypeId::of::<T>();
                let comp_error = ResourceError { message: format!("Failed to read resource with type id {type_id:?}: {err:?}") };
                Err(comp_error)
            }
        }
    }

    fn get_mut(&mut self) -> Result<RwLockWriteGuard<Self::Object>, ResourceError>
    {
        match self.data.write()
        {
            Ok(data) => Ok(data),
            Err(err) => 
            {
                let type_id = TypeId::of::<T>();
                let comp_error = ResourceError { message: format!("Failed to write resource with type id {type_id:?}: {err:?}") };
                Err(comp_error)
            }
        }
    }
}

pub struct Resources
{
    // data: HashMap<TypeId, Arc<RwLock<dyn Any>>>,
    data: HashMap<TypeId, Box<dyn Any>>
}

impl Resources
{
    pub fn new() -> Self
    {
        Self
        {
            data: HashMap::new()
        }
    }

    pub fn add<T: Any>(&mut self, data: T)
    {
        let type_id = data.type_id();
        // self.data.insert(type_id, Arc::new(RwLock::new(data)));
        // self.data.insert(type_id, Box::new(data));
        self.data.insert(type_id, Box::new(Resource::new(data)));
    }

    // pub fn get<T: Any>(&self) -> Option<&Box<T>>
    pub fn get<T: Send + Sync + 'static>(&self) -> Result<RwLockReadGuard<T>, ResourceError>
    {
        let type_id = TypeId::of::<T>();
        // if let Some(data) = self.data.get(&type_id)
        // {
        //     data.downcast_ref::<Box<T>>()
        // } else {
        //     None
        // }
        match self.data.get(&type_id)
        {
            Some(data) => 
            {
                let d = data.downcast_ref::<Resource<T>>().expect(format!("Failed to get resource of type {type_id:?}").as_str());
                Ok(d.get()?)
            },
            None => Err(ResourceError { message: format!("Failed to get resource of type {type_id:?}") })
        }
    }

    // pub fn get_mut<T: Any>(&mut self) -> Option<&mut T>
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Result<RwLockWriteGuard<T>, ResourceError>
    {
        let type_id = TypeId::of::<T>();
        // if let Some(data) = self.data.get_mut(&type_id)
        // {
        //     data.downcast_mut::<T>()
        // } else {
        //     None
        // }
        match self.data.get_mut(&type_id)
        {
            Some(data) => 
            {
                let d = data.downcast_mut::<Resource<T>>().expect(format!("Failed to get resource of type {type_id:?}").as_str());
                Ok(d.get_mut()?)
            },
            None => Err(ResourceError { message: format!("Failed to get resource of type {type_id:?}") })
        }
    }

    pub fn remove<T: Any>(&mut self)
    {
        let type_id = TypeId::of::<T>();
        self.data.remove(&type_id);
    }
    
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_resources()
    {
        let mut resources = Resources::new();
        resources.add(DeltaTime{ value: 1.0 });
        resources.add(ScreenSize{ width: 100.0, height: 100.0 });

        assert!(resources.get::<DeltaTime>().is_ok());
        assert_eq!(resources.get::<DeltaTime>().unwrap().value, 1.0);

        let mut delta_time = resources.get_mut::<DeltaTime>().unwrap();
        delta_time.value = 5.0;
        assert_eq!(delta_time.value, 5.0);
    }

    pub struct DeltaTime
    {
        pub value: f32
    }

    pub struct ScreenSize
    {
        pub width: f32,
        pub height: f32
    }
}