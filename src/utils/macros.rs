#[macro_export]
macro_rules! unwrap_error {
    ($result:expr, $message:expr) => {{
        let caller = std::panic::Location::caller().to_string();
        $result.unwrap_or_else(|error| panic!("\n{}\n{}\nCalled at {}\n", $message, error, caller))
    }};
}

#[macro_export]
macro_rules! print_error {
    ($result:expr, $message:expr) => {{
        let caller = std::panic::Location::caller().to_string();
        let _ =
            $result.map_err(|error| println!("\n{}\n{}\nCalled at {}\n", $message, error, caller));
    }};
}
