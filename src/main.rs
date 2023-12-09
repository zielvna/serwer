use serwer::{
    enums::{Method, StatusCode},
    structs::{Response, Route, Serwer},
};

fn main() {
    let mut serwer = Serwer::new();

    serwer.add_route(
        Route::new(Method::GET, "ok", |request| {
            let method = request.get_method().to_string();

            Response::new(StatusCode::OK, format!("{method} ok"))
        })
        .unwrap(),
    );
    serwer.add_route(
        Route::new(Method::POST, "not-found", |request| {
            let method = request.get_method().to_string();

            Response::new(StatusCode::NotFound, format!("{method} not found"))
        })
        .unwrap(),
    );

    serwer.listen(7878);
}
