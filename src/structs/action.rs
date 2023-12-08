pub struct Action {
    func: Box<dyn Fn() -> String>,
}

impl Action {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn() -> String + 'static,
    {
        Self {
            func: Box::new(func),
        }
    }

    pub fn run(&self) -> String {
        (self.func)()
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}
