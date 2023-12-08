use crate::{
    enums::{HTTPMethod, SerwerError},
    utils::validate_path,
};

use super::Action;

#[derive(Debug)]
pub struct Route {
    method: HTTPMethod,
    path: &'static str,
    action: Action,
}

impl Route {
    pub fn new(
        method: HTTPMethod,
        path: &'static str,
        action: Action,
    ) -> Result<Self, SerwerError> {
        validate_path(&path)?;

        Ok(Self {
            method,
            path,
            action,
        })
    }

    pub fn get_method(&self) -> &HTTPMethod {
        &self.method
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn run_action(&self) -> String {
        self.action.run()
    }
}
