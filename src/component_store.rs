use std::any::Any;



pub trait ComponentStore
{
    // type DataType;

    fn push_none(&mut self);

    fn set_none(&mut self, index: usize);

    fn resize_to_nones(&mut self, len: usize);

    fn drop(&mut self, index: usize);
    
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    // fn set(&mut self, index: usize, data: Option<impl Any>);

    // fn get(&self, index: usize) -> Option<&impl Any>;

    // fn get_mut(&mut self, index: usize) -> Option<&mut impl Any>;


}

pub struct VecStore<T>
{
    data: Vec<Option<T>>
}

impl <T> VecStore<T>
{
    pub fn new() -> Self
    {
        Self { data: Vec::new() }
    }

    pub fn set(&mut self, index: usize, data: Option<T>)
    {
        self.data[index] = data;
    }

    pub fn len(&self) -> usize
    {
        self.data.len()
    }
}

impl<T: 'static> ComponentStore for VecStore<T>
{
    // type DataType = T;

    fn push_none(&mut self)
    {
        self.data.push(None)
    }

    fn set_none(&mut self, index: usize) 
    {
        self.data[index] = None;
    }
    
    fn resize_to_nones(&mut self, len: usize)
    {
        self.data.resize_with(len, || { None } );
    }

    fn drop(&mut self, index: usize)
    {
        self.data[index] = None;
    }

    fn as_any(&self) -> &dyn Any
    {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any
    {
        self as &mut dyn std::any::Any
    }

    // fn set(&mut self, index: usize, data: Option<impl Any>)
    // {
    //     self.data[index] = data;
    // }

    // fn get(&self, index: usize) -> Option<&Self::DataType> 
    // {
    //     self.data[index].as_ref()
    // }

    // fn get_mut(&mut self, index: usize) -> Option<&mut Self::DataType>
    // {
    //     self.data[index].as_mut()
    // }


    // fn resize_with(&mut self, len: usize, f: impl Fn() -> Option<Self::DataType>)
    // {
    //     self.data.resize_with(len, f);
    // }
}