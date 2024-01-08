use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Clone)]
pub struct Data<T> {
    data: Arc<RwLock<T>>,
}

impl<T> Data<T> {
    pub fn new(initial_data: T) -> Self {
        Self {
            data: Arc::new(RwLock::new(initial_data)),
        }
    }

    pub fn get(&self) -> Arc<RwLock<T>> {
        self.data.clone()
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        let data = self.data.read().unwrap();
        data
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        let data = self.data.write().unwrap();
        data
    }
}
