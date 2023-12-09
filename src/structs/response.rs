use crate::enums::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: String,
}

impl Response {
    pub fn new(status_code: StatusCode, body: String) -> Self {
        Self { status_code, body }
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn get_status_code(&self) -> StatusCode {
        self.status_code.clone()
    }
}
