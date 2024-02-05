use crate::unwrap_error;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

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

impl<T> Clone for Data<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = Data::new(42);
        assert_eq!(*data.read(), 42);
    }

    #[test]
    fn test_get() {
        let data = Data::new(42);
        let data_clone = data.get();
        assert_eq!(*data_clone.read().unwrap(), 42);
    }

    #[test]
    fn test_read() {
        let data = Data::new(42);
        assert_eq!(*data.read(), 42);
    }

    #[test]
    fn test_write() {
        let data = Data::new(42);
        *data.write() = 43;
        assert_eq!(*data.read(), 43);
    }

    #[test]
    fn test_clone() {
        let data = Data::new(42);
        let data_clone = data.clone();
        *data_clone.write() = 43;
        assert_eq!(*data.read(), 43);
    }
}
