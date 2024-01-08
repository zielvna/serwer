use crate::unwrap_error;
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
        unwrap_error!(self.data.read(), "Failed to lock data for read access")
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        unwrap_error!(self.data.write(), "Failed to lock data for write access")
    }
}
