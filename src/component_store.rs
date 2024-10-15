use std::{any::Any, fmt, error};
pub use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

// use anyhow;

pub struct ComponentError
{
    // code: usize,
    message: String
}

impl fmt::Display for ComponentError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for ComponentError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "{}", self.message)
    }
}

impl<T: error::Error + Send + Sync + 'static> From<T> for ComponentError 
{
    fn from(e: T) -> Self 
    {
        Self { message: e.to_string()}
    }
}


pub trait ComponentStore: Send + Sync
{
    fn push_none(&mut self);

    fn set_none(&mut self, index: usize);

    fn resize_to_nones(&mut self, len: usize);

    fn drop(&mut self, index: usize);
    
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct VecStore<T>
{
    data: Vec<RwLock<Option<T>>>
}

impl <T> VecStore<T>
{
    pub fn new() -> Self
    {
        Self { data: Vec::new() }
    }

    pub fn get(&self, index: usize) -> Result<RwLockReadGuard<Option<T>>, ComponentError>
    {
        match self.data[index].read()
        {
            Ok(data) => Ok(data),
            Err(err) => 
            {
                let comp_error = ComponentError { message: format!("Failed to get component at entity id {index}: {err:?}") };
                Err(comp_error)
            }
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Result<RwLockWriteGuard<Option<T>>, ComponentError>
    {
        match self.data[index].write()
        {
            Ok(data) => Ok(data),
            Err(err) => 
            {
                let comp_error = ComponentError { message: format!("Failed to get mut component at entity id {index}: {err:?}") };
                Err(comp_error)
            }
        }
    }

    pub fn len(&self) -> usize
    {
        self.data.len()
    }

}

impl<T: Send + Sync + 'static> ComponentStore for VecStore<T>
{
    fn push_none(&mut self)
    {
        self.data.push(RwLock::new(None));
    }

    fn set_none(&mut self, index: usize) 
    {
        let mut val = self.data[index].write().unwrap();
        *val = None;
    }
    
    fn resize_to_nones(&mut self, len: usize)
    {
        self.data.resize_with(len, || { RwLock::new(None) } );
    }

    fn drop(&mut self, index: usize)
    {
        let mut val = self.data[index].write().unwrap();
        *val = None;
    }

    fn as_any(&self) -> &dyn Any
    {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any
    {
        self as &mut dyn std::any::Any
    }

}