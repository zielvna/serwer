use serwer::{enums::StatusCode, structs::Serwer};

fn main() {
    let mut serwer = Serwer::new();

    serwer.public("public");

    serwer.get("/user/<id>", |req, mut res| {
        let id = req.get_param("id").unwrap();

        res.set_header("X-Test", "test");
        res.set_cookie("session_id", "v82t9j3gb9s1y3xta9a0nvesj8qilu9z");
        res.set(StatusCode::OK, format!("user id: {id}"))
    });

    serwer.listen(7878);
}
