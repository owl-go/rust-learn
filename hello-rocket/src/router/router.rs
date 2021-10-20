use rocket::{fs::NamedFile, tokio::task::spawn_blocking};
use std::{
    io,
    path::{Path, PathBuf},
};
// #[macro_use]
// extern crate rocket;

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes!(index))
// }
#[get("/<file..>")] //读取文件
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[post("/<id>")]
pub fn post_ex(id: usize) -> String {
    format!("get id:{}", id)
}

#[get("/page/<path..>")]
pub fn get_page(path: PathBuf) -> String {
    if let Some(s) = path.to_str() {
        s.to_string()
    } else {
        "page".to_string()
    }
}

#[get("/blocking_task")]
pub async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[catch(404)]
pub fn not_found() -> &'static str {
    "404 not found!"
}
