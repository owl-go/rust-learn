mod router;
use std::collections::HashMap;

use rocket::fs::FileServer;
use router::index::{foo, get_json, hello, index, task};
use router::router::{blocking_task, files, get_page, not_found, post_ex};
use router::user::{create_user, get_user, user_str, UserItem};
#[macro_use]
extern crate rocket;
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .manage(UserItem::new(HashMap::new()))
        //路由
        .mount(
            "/",
            routes![index, get_page, blocking_task, task, get_json, hello, foo,],
        )
        .mount("/user", routes![get_user, create_user, user_str])
        .mount("/ex", routes!(post_ex))
        // .mount("/public", routes![files])
        .mount("/public", FileServer::from("static")) //读取静态文件
        //catcher
        .register("/", catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
