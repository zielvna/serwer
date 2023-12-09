use serwer::{enums::StatusCode, structs::Serwer};

fn main() {
    let mut serwer = Serwer::new();

    serwer.get("ok", |req, mut res| {
        let method = req.get_method().to_string();

        res.set(StatusCode::OK, format!("{method} ok"))
    });

    serwer.post("not-found", |req, mut res| {
        let method = req.get_method().to_string();

        res.set(StatusCode::OK, format!("{method} not found"))
    });

    serwer.listen(7878);
}
