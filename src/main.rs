use serwer::{HTTPMethod, Route, Serwer};

fn main() {
    let mut serwer = Serwer::new();
    println!("{:?}", serwer);
    serwer.add_route(Route::new(HTTPMethod::GET, String::from("/")));
    println!("{:?}", serwer);
    serwer.listen(7878);
    println!("{:?}", serwer);
}
