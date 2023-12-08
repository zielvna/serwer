use serwer::{Action, HTTPMethod, Route, Serwer};

fn main() {
    let mut serwer = Serwer::new();
    println!("{:?}", serwer);
    serwer.add_route(Route::new(
        HTTPMethod::GET,
        String::from("/test"),
        Action::new(|| String::from("test")),
    ));
    serwer.add_route(Route::new(
        HTTPMethod::GET,
        String::from("/test2"),
        Action::new(|| String::from("test2")),
    ));
    println!("{:?}", serwer);
    serwer.listen(7878);
    println!("{:?}", serwer);
}
