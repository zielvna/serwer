use crate::enums::StatusCode;

#[derive(Clone)]
pub struct Response {
    status_code: StatusCode,
    body: String,
}

impl Response {
    pub fn new(status_code: StatusCode, body: String) -> Self {
        Self { status_code, body }
    }

    pub fn get(&self) -> (StatusCode, String) {
        (self.status_code.clone(), self.body.clone())
    }

    pub fn set(&mut self, status_code: StatusCode, body: String) -> Self {
        self.status_code = status_code;
        self.body = body;

        self.clone()
    }

    pub fn get_status_code(&self) -> StatusCode {
        self.status_code.clone()
    }

    pub fn set_status_code(&mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;

        self.clone()
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn set_body(&mut self, body: String) -> Self {
        self.body = body;

        self.clone()
    }
}
