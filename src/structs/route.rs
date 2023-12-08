use crate::enums::HTTPMethod;

use super::Action;

#[derive(Debug)]
pub struct Route {
    method: HTTPMethod,
    path: String,
    action: Action,
}

impl Route {
    pub fn new(method: HTTPMethod, path: String, action: Action) -> Self {
        Self {
            method,
            path,
            action,
        }
    }

    pub fn get_method(&self) -> &HTTPMethod {
        &self.method
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn run_action(&self) -> String {
        self.action.run()
    }
}
