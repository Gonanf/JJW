use rocket_dyn_templates::Template;
use local_ip_address::local_ip;
use rocket::{serde::{self, Deserialize, Serialize}, tokio::sync::broadcast::channel};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str{

 println!("Amongusw sex {}", local_ip().unwrap());
 let mut i: i8 = 0;
 while i <= 10{
    println!("num: {}", i);
    i+=1;
 }
 "Amongas"
}

#[derive(Debug, Clone, Serialize, Deserialize, )]
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
    .attach(Template::fairing())
}