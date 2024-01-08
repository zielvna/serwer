use json::{object, JsonValue};
use serwer::{route, Data, Serwer, StatusCode};
use std::{
    fs,
    sync::atomic::{AtomicUsize, Ordering},
};

fn main() {
    let mut serwer = Serwer::new();

    let todo_list: Data<Vec<JsonValue>> = Data::new(vec![]);

    serwer.get(
        "/tasks",
        route! {(todo_list) move |_, mut res| {
            let todo_list = todo_list.read();
            res.set(StatusCode::OK, json::stringify(todo_list.as_slice()));
            res
        }},
    );

    serwer.get(
        "/task/<id>",
        route! {(todo_list) move |req, mut res| {
            let todo_list = todo_list.read();
            let id = req.get_param("id").unwrap_or_default();

            for task in todo_list.iter() {
                if task["id"].to_string() == id {
                    res.set(StatusCode::OK, json::stringify(task.clone()));
                    return res;
                }
            }

            res.set(StatusCode::NotFound, (object! {
                status: "error",
                message: "Task not found"
            }).dump());
            res

        }},
    );

    serwer.get(
        "/",
        route! {() move |_, mut res| {
            let body = fs::read_to_string("static/index.html").unwrap();
            res.set(StatusCode::OK, body);
            res
        }},
    );

    serwer.post(
        "/task",
        route! {(todo_list) move |req, mut res| {
            let mut todo_list = todo_list.write();
            let mut task = json::parse(req.get_body().unwrap().as_str()).unwrap();

            if task["description"].is_null() {
                res.set(StatusCode::BadRequest, (object! {
                    status: "error",
                    message: "Task description is required"
                }).dump());
                return res;
            }

            task["id"] = get_id().into();
            task["completed"] = false.into();

            todo_list.push(task);

            res.set(StatusCode::Created, (object! {
                status: "success",
                message: "Task added successfully"
            }).dump());
            res
        }},
    );

    serwer.patch(
        "/task/<id>",
        route! {(todo_list) move |req, mut res| {
            let mut todo_list = todo_list.write();
            let id = req.get_param("id").unwrap_or_default();
            let completed = json::parse(req.get_body().unwrap().as_str()).unwrap();

            if completed["completed"].is_null() || !completed["completed"].is_boolean() {
                res.set(StatusCode::BadRequest, (object! {
                    status: "error",
                    message: "Completed field is required"
                }).dump());
                return res;
            }

            for task in todo_list.iter_mut() {
                if task["id"].to_string() == id {
                    task["completed"] = completed["completed"].clone().into();
                    res.set(StatusCode::Accepted, (object! {
                        status: "success",
                        message: "Task updated successfully"
                    }).dump());
                    return res;
                }
            }

            res.set(StatusCode::NotFound, (object! {
                status: "error",
                message: "Task not found"
            }).dump());
            res
        }},
    );

    serwer.delete(
        "/task/<id>",
        route! {(todo_list) move |req, mut res| {
            let mut todo_list = todo_list.write();
            let id = req.get_param("id").unwrap_or_default();

            for (index, task) in todo_list.iter_mut().enumerate() {
                if task["id"].to_string() == id {
                    todo_list.remove(index);
                    res.set(StatusCode::Accepted, (object! {
                        status: "success",
                        message: "Task deleted successfully"
                    }).dump());
                    return res;
                }
            }

            res.set(StatusCode::NotFound, (object! {
                status: "error",
                message: "Task not found"
            }).dump());
            res
        }},
    );

    serwer.listen(7878);
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn get_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
