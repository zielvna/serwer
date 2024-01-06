macro_rules! unwrap_error {
    ($result:expr, $message:expr) => {{
        let caller = std::panic::Location::caller().to_string();
        $result.unwrap_or_else(|error| panic!("\n{}\n{}\nCalled at {}\n", $message, error, caller))
    }};
}

macro_rules! unwrap_none {
    ($result:expr, $message:expr) => {{
        let caller = std::panic::Location::caller().to_string();
        $result.unwrap_or_else(|| panic!("\n{}\nCalled at {}\n", $message, caller))
    }};
}

macro_rules! print_error {
    ($result:expr, $message:expr) => {{
        let caller = std::panic::Location::caller().to_string();
        let _ =
            $result.map_err(|error| println!("\n{}\n{}\nCalled at {}\n", $message, error, caller));
    }};
}

macro_rules! generate_route {
    ($method: ident, $method_enum: expr) => {
        #[track_caller]
        pub fn $method<F>(&mut self, path: &'static str, action: F)
        where
            F: Fn(Request, Response) -> Response + Send + Sync + 'static,
        {
            let routes = Arc::clone(&self.routes);
            let mut routes = routes.write().expect("Error while locking routes");

            routes.push(unwrap_error!(
                Route::new($method_enum, path, action),
                "Error while setting route"
            ));
        }
    };
}

pub(crate) use generate_route;
pub(crate) use print_error;
pub(crate) use unwrap_error;
pub(crate) use unwrap_none;
