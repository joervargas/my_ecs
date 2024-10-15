mod entity;
mod entity_builder;
mod component_store;
mod resource;
mod registry;
mod system;
mod query;
mod world;

// mod tuple_append;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
