use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    id: usize,
    name: String,
}

#[get("/<param>")]
pub fn index(param: isize) -> String {
    format!("/{}", param)
}

#[post("/task", format = "json", data = "<task>")] //post json
pub fn task(task: Json<Task>) -> Json<Task> {
    let task = task.into_inner();
    Json(Task {
        id: task.id,
        // name: format!("{}", task.name),
        name: task.name,
    })
}

#[get("/get_json")]
pub fn get_json() -> Value {
    json!({"res":0,"name":"hello"})
}

#[get("/hello/<name>/<age>/<cool>")]
pub fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/foo/<_>/bar")]
pub fn foo() -> &'static str {
    "foo_bar"
}
