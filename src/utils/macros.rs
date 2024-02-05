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
            let mut routes =
                unwrap_error!(routes.write(), "Failed to lock routes for write access");

            routes.push(unwrap_error!(
                Route::new($method_enum, path, action),
                "Error while setting route"
            ));
        }
    };
}

#[macro_export]
macro_rules! route {
    (($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub(crate) use generate_route;
pub(crate) use print_error;
pub(crate) use unwrap_error;
pub(crate) use unwrap_none;

#[cfg(test)]
mod tests {
    #[test]
    fn test_unwrap_error() {
        let result = unwrap_error!(Ok::<String, String>(String::from("Ok")), "Error message");
        assert_eq!(result, String::from("Ok"));
    }

    #[test]
    #[should_panic]
    fn test_unwrap_error_should_panic() {
        unwrap_error!(Err::<String, String>(String::from("Err")), "Error message");
    }

    #[test]
    fn test_unwrap_none() {
        let result = unwrap_none!(Some::<String>(String::from("Some")), "Error message");
        assert_eq!(result, String::from("Some"));
    }

    #[test]
    #[should_panic]
    fn test_unwrap_none_should_panic() {
        unwrap_none!(None, "Error message");
    }
}
