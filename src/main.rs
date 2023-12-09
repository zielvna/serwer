use serwer::{enums::StatusCode, structs::Serwer};

fn main() {
    let mut serwer = Serwer::new();

    serwer.get("/user/<id>", |req, mut res| {
        let id = req.get_param("id").unwrap();

        res.set(StatusCode::OK, format!("user id: {id}"))
    });

    serwer.listen(7878);
}
