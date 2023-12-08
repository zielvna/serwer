use serwer::{
    enums::HTTPMethod,
    structs::{Action, Route, Serwer},
};

fn main() {
    let mut serwer = Serwer::new();
    println!("{:?}", serwer);
    serwer.add_route(
        Route::new(
            HTTPMethod::GET,
            "test",
            Action::new(|| String::from("test")),
        )
        .unwrap(),
    );
    serwer.add_route(
        Route::new(
            HTTPMethod::GET,
            "test2",
            Action::new(|| String::from("test2")),
        )
        .unwrap(),
    );
    println!("{:?}", serwer);
    serwer.listen(7878);
    println!("{:?}", serwer);
}
