use std::path::{Path, PathBuf};
use rocket::{form::name::Name, fs::{relative, FileServer}};
use rocket_dyn_templates::{context, Template};
use rocket::{fs::NamedFile, serde::{Deserialize, Serialize}, tokio::sync::broadcast::channel};

#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Option<NamedFile>{
    NamedFile::open("files/html/main.html").await.ok()
}


#[derive(Debug, FromForm , Clone, Serialize, Deserialize, )]
#[serde(crate = "rocket::serde")]
struct Sesion{
    usuario: String,
    destino: String,
    mensaje: String
}
#[launch]
fn rocket() -> _{
    rocket::build()
    .manage(channel::<Sesion>(500).0)
    .mount("/", routes![index])
    .mount("/", FileServer::from(relative!("files")))
    .attach(Template::fairing())
}