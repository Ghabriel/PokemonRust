use std::{any::Any, collections::HashMap};

#[derive(Default)]
pub struct PolymorphicContainer {
    data: HashMap<usize, Box<dyn Any>>,
}

impl PolymorphicContainer {
    pub fn store(&mut self, value: impl Any) -> usize {
        let key = self.data.len();
        self.data.insert(key, Box::new(value));
        key
    }

    pub fn store_at(&mut self, key: usize, value: Box<dyn Any>) {
        self.data.insert(key, value);
    }

    pub fn try_remove_boxed(&mut self, key: usize) -> Option<Box<dyn Any>> {
        self.data.remove(&key)
    }

    pub fn remove_boxed(&mut self, key: usize) -> Box<dyn Any> {
        self.try_remove_boxed(key).unwrap()
    }

    pub fn try_remove<T: 'static>(&mut self, key: usize) -> Option<Box<T>> {
        self.data.remove(&key)?.downcast().ok()
    }

    pub fn remove<T: 'static>(&mut self, key: usize) -> Box<T> {
        self.try_remove(key).unwrap()
    }
}
