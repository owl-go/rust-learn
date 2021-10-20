use rocket::serde::json::Json;
use std::collections::HashMap;

use rocket::{
    serde::{Deserialize, Serialize},
    tokio::sync::Mutex,
    State,
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Person {
    id: usize,
    name: String,
    age: u8,
}

pub type UserItem = Mutex<HashMap<usize, Person>>;
pub type Message<'r> = &'r State<UserItem>;

#[get("/search/<id>")]
pub async fn get_user(id: usize, message: Message<'_>) -> Json<Person> {
    let user_map = message.lock().await;
    if id == 0 {
        return Json(Person {
            id: 0,
            name: "".to_string(),
            age: 0,
        });
    }
    match user_map.get(&id) {
        None => Json(Person {
            id: 0,
            name: "None".to_string(),
            age: 0,
        }),
        Some(u) => Json(u.to_owned()),
    }
}

#[post("/create", format = "json", data = "<person>")]
pub async fn create_user(person: Json<Person>, message: Message<'_>) -> String {
    let mut user_map = message.lock().await;
    let new_person = person.into_inner();
    if user_map.contains_key(&new_person.id) {
        format!("user exist {}", new_person.id)
    } else {
        let id = new_person.id;
        user_map.insert(new_person.id, new_person);
        format!("create user success:{}", id)
    }
}

#[delete("/del/<id>", rank = 3)]
pub async fn user_str(id: usize, message: Message<'_>) -> String {
    let mut user_map = message.lock().await;
    if user_map.contains_key(&id) {
        user_map.remove(&id);
        "remove success".to_string()
    } else {
        format!("person not exist:{}", id)
    }
}
