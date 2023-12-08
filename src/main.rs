use serwer::{
    enums::Method,
    structs::{Route, Serwer},
};

fn main() {
    let mut serwer = Serwer::new();
    println!("{:?}", serwer);
    serwer.add_route(Route::new(Method::GET, "home", || String::from("home")).unwrap());
    serwer.add_route(Route::new(Method::POST, "login", || String::from("login")).unwrap());
    println!("{:?}", serwer);
    serwer.listen(7878);
    println!("{:?}", serwer);
}
