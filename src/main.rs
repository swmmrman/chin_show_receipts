use rocket::fs::FileServer;
use rocket::tokio::fs;
use rocket::response::content;
use std::path::{Path,PathBuf};

#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> content::RawHtml<String>{
    makepage().await
}

async fn makepage() -> content::RawHtml<String> {
    let mut page: String = String::new();
    let header: String = fs::read_to_string(Path::new("templates").join("head.html")).await.unwrap();
    let body: String = fs::read_to_string(Path::new("templates").join("main.html")).await.unwrap();
    let footer: String = fs::read_to_string(Path::new("templates").join("footer.html")).await.unwrap();
    page = format!("{}{}{}{}",page,header,body,footer);
    content::RawHtml(page)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/css", FileServer::from("html/css"))
}