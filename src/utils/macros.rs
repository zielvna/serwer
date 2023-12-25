#[macro_export]
macro_rules! unwrap {
    ($result:expr, $message:expr) => {{
        let caller = std::panic::Location::caller().to_string();
        $result.unwrap_or_else(|error| panic!("\n{}\n{}\nCalled at {}\n", $message, error, caller))
    }};
}

pub(crate) use unwrap;
