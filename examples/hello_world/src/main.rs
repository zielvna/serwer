use serwer::{route, Serwer, StatusCode};

fn main() {
    let mut serwer = Serwer::new();

    serwer.get(
        "/",
        route! {() move |_, mut res| {
            res.set(StatusCode::OK, "Hello world".to_string());
            res
        }},
    );

    serwer.listen(7878);
}
