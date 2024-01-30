use bcrypt::{hash, DEFAULT_COST};
use json::object;
use rand::{distributions::Alphanumeric, Rng};
use serwer::{route, Cookie, Data, Serwer, StatusCode};
use sqlite::Connection;
use std::fs;

struct User {
    id: i64,
    username: String,
    password: String,
}

struct Session {
    _id: String,
    user_id: i64,
}

fn main() {
    let connection = sqlite::Connection::open_thread_safe(":memory:").unwrap();

    let mut serwer = Serwer::new();

    let query = "
        CREATE TABLE users (id INTEGER PRIMARY KEY, username TEXT, password TEXT);
        CREATE TABLE posts (id INTEGER PRIMARY KEY, user_id INTEGER, text TEXT);
        CREATE TABLE sessions (id INTEGER, user_id INTEGER);
    ";
    connection.execute(query).unwrap();

    let connection = Data::new(connection);

    serwer.post(
        "/signin",
        route! {(connection) move |req, mut res| {
            let connection = connection.read();
            let data = json::parse(req.body().unwrap().as_str()).unwrap();

            if data["username"].is_null() || data["password"].is_null() {
                res.set(StatusCode::BadRequest, (object! {
                    status: "error",
                    message: "Missing username or password"
                }).dump());
                return res;
            }

            let user = match get_user_by_username(&connection, &data["username"].to_string()) {
                Some(user) => user,
                None => {
                    res.set(StatusCode::BadRequest, (object! {
                        status: "error",
                        message: "User does not exist"
                    }).dump());
                    return res;
                }
            };

            let password_match = bcrypt::verify(data["password"].to_string(), &user.password).unwrap();

            if !password_match {
                res.set(StatusCode::Unauthorized, (object! {
                    status: "error",
                    message: "Password do not match"
                }).dump());
                return res;
            }

            remove_session(&connection, user.id);
            let session_id = generate_session_id();
            add_session(&connection, &session_id, user.id);
            res.set_cookie("session_id", Cookie::new("session_id", &session_id).set_max_age(1000 * 60 * 60 * 3));

            res.set(StatusCode::OK, (object! {
                status: "success",
                message: "User successfully signed in"
            }).dump());
            res
        }},
    );

    serwer.post(
        "/signup",
        route! {(connection) move |req, mut res| {
            let connection = connection.read();
            let data = json::parse(req.body().unwrap().as_str()).unwrap();

            if data["username"].is_null() || data["username"] == "" || data["password"].is_null() || data["password"] == "" {
                res.set(StatusCode::BadRequest, (object! {
                    status: "error",
                    message: "Missing username or password"
                }).dump());
                return res;
            }

            let user = get_user_by_username(&connection, &data["username"].to_string());

            if user.is_some() {
                res.set(StatusCode::BadRequest, (object! {
                    status: "error",
                    message: "Username already exists"
                }).dump());
                return res;
            }

            let hashed_password = hash(&data["password"].to_string(), DEFAULT_COST).unwrap();
            add_user(&connection, &data["username"].to_string(), &hashed_password);

            let user = get_user_by_username(&connection, &data["username"].to_string()).unwrap();
            let session_id = generate_session_id();
            add_session(&connection, &session_id, user.id);
            res.set_cookie("session_id", Cookie::new("session_id", &session_id).set_max_age(1000 * 60 * 60 * 3));

            res.set(StatusCode::Created, (object! {
                status: "success",
                message: "User successfully signed up"
            }).dump());
            res
        }},
    );

    serwer.get(
        "/signout",
        route! {(connection) move |req, mut res| {
            let connection = connection.read();

            let cookie = match req.cookie("session_id") {
                Some(cookie) => cookie,
                None => {
                    res.set(StatusCode::Unauthorized, (object! {
                        status: "error",
                        message: "User is not signed in"
                    }).dump());
                    return res;
                }
            };

            let session = match get_session(&connection, cookie.value()) {
                Some(session) => session,
                None => {
                    res.set(StatusCode::Unauthorized, (object! {
                        status: "error",
                        message: "Session not found"
                    }).dump());
                    return res;
                }
            };

            remove_session(&connection, session.user_id);
            res.set_cookie("session_id", Cookie::new("session_id", "").set_max_age(0));

            res.set(StatusCode::OK, (object! {
                status: "success",
                message: "User successfully signed out"
            }).dump());
            res
        }},
    );

    serwer.get(
        "/user",
        route! {(connection) move |req, mut res| {
            let connection = connection.read();

            let cookie = match req.cookie("session_id") {
                Some(cookie) => cookie,
                None => {
                    res.set(StatusCode::Unauthorized, (object! {
                        status: "error",
                        message: "User is not signed in"
                    }).dump());
                    return res;
                }
            };

            let session = match get_session(&connection, cookie.value()) {
                Some(session) => session,
                None => {
                    res.set(StatusCode::Unauthorized, (object! {
                        status: "error",
                        message: "Session not found"
                    }).dump());
                    return res;
                }
            };

            let user = match get_user_by_id(&connection, session.user_id) {
                Some(user) => user,
                None => {
                    res.set(StatusCode::Unauthorized, (object! {
                        status: "error",
                        message: "User not found"
                    }).dump());
                    return res;
                }
            };

            res.set(StatusCode::OK, json::stringify(object! { status: "success", message: { id: user.id, username: user.username }}));
            res
        }},
    );

    serwer.get(
        "/",
        route! {(connection) move |req, mut res| {
            let connection = connection.read();

            let cookie = match req.cookie("session_id") {
                Some(cookie) => cookie,
                None => {
                    let body = fs::read_to_string("static/index.html").unwrap();
                    res.set(StatusCode::OK, body);
                    return res;
                }
            };

            if get_session(&connection, cookie.value()).is_none() {
                let body = fs::read_to_string("static/index.html").unwrap();
                res.set(StatusCode::OK, body);
                return res;
            }

            res.set_status_code(StatusCode::SeeOther);
            res.set_header("Location", "/home");
            res
        }},
    );

    serwer.get(
        "/home",
        route! {(connection) move |req, mut res| {
            let connection = connection.read();

            let cookie = match req.cookie("session_id") {
                Some(cookie) => cookie,
                None => {
                    res.set_status_code(StatusCode::SeeOther);
                    res.set_header("Location", "/");
                    return res;
                }
            };

            if get_session(&connection, cookie.value()).is_none() {
                res.set_status_code(StatusCode::SeeOther);
                res.set_header("Location", "/");
                return res;
            }

            let body = fs::read_to_string("static/home.html").unwrap();
            res.set(StatusCode::OK, body);
            res
        }},
    );

    serwer.listen(7878);
}

fn add_user(connection: &Connection, username: &str, password: &str) {
    let mut statement = connection
        .prepare("INSERT INTO users (username, password) VALUES (?, ?);")
        .unwrap();
    statement.bind((1, username)).unwrap();
    statement.bind((2, password)).unwrap();

    statement.next().unwrap();
}

fn get_user_by_username(connection: &Connection, username: &str) -> Option<User> {
    let mut statement = connection
        .prepare("SELECT * FROM users WHERE username = ?;")
        .unwrap();
    statement.bind((1, username)).unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        let id = statement.read::<i64, _>("id").unwrap();
        let username = statement.read::<String, _>("username").unwrap();
        let password = statement.read::<String, _>("password").unwrap();

        Some(User {
            id,
            username,
            password,
        })
    } else {
        None
    }
}

fn get_user_by_id(connection: &Connection, id: i64) -> Option<User> {
    let mut statement = connection
        .prepare("SELECT * FROM users WHERE id = ?;")
        .unwrap();
    statement.bind((1, id)).unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        let id = statement.read::<i64, _>("id").unwrap();
        let username = statement.read::<String, _>("username").unwrap();
        let password = statement.read::<String, _>("password").unwrap();

        Some(User {
            id,
            username,
            password,
        })
    } else {
        None
    }
}

fn add_session(connection: &Connection, session_id: &str, user_id: i64) {
    let mut statement = connection
        .prepare("INSERT INTO sessions (id, user_id) VALUES (?, ?);")
        .unwrap();
    statement.bind((1, session_id)).unwrap();
    statement.bind((2, user_id)).unwrap();

    statement.next().unwrap();
}

fn get_session(connection: &Connection, session_id: &str) -> Option<Session> {
    let mut statement = connection
        .prepare("SELECT * FROM sessions WHERE id = ?;")
        .unwrap();
    statement.bind((1, session_id)).unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        let id = statement.read::<String, _>("id").unwrap();
        let user_id = statement.read::<i64, _>("user_id").unwrap();

        Some(Session { _id: id, user_id })
    } else {
        None
    }
}

fn remove_session(connection: &Connection, user_id: i64) {
    let mut statement = connection
        .prepare("DELETE FROM sessions WHERE user_id = ?;")
        .unwrap();
    statement.bind((1, user_id)).unwrap();

    statement.next().unwrap();
}

fn generate_session_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
