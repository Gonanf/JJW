use rocket::form::{Form, FromForm};
use rocket::fs::{relative, FileServer};
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::error;
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::{fs::NamedFile, serde::{Deserialize, Serialize}, State, Shutdown};

#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Option<NamedFile>{
    NamedFile::open("files/html/main.html").await.ok()
}

#[get("/lobby")]
async fn lobby() -> Option<NamedFile>{
    NamedFile::open("files/html/lobby.html").await.ok()
}


#[derive(Debug, FromForm , Clone, Serialize, Deserialize, )]
#[serde(crate = "rocket::serde")]
struct Sesion{
    usuario: String,
    destino: String,
    mensaje: String
}

#[post("/mensaje", data = "<ms>")]
fn msg(ms: Form<Sesion>,q: &State<Sender<Sesion>>){
    let _f = q.send(ms.into_inner());
}

#[get("/server")]
async fn server(q: &State<Sender<Sesion>>, mut t: Shutdown) -> EventStream![]{
    let mut subs = q.subscribe();
    EventStream! {
        loop {
            let serv_m = select! {
                serv_m = subs.recv() => match serv_m {
                    Ok(serv_m) => {
                        println!("Todo bein");
                        serv_m},
                    Err(RecvError::Closed) => {
                        println!("Error, se corto la conexion");
                        break},
                    Err(RecvError::Lagged(_)) => {
                        println!("Lageo");
                        continue},
                },
                _ = &mut t => break,
            };

            yield Event::json(&serv_m);
        }
    }


}

#[launch]
fn rocket() -> _{
    rocket::build()
    .manage(channel::<Sesion>(500).0)
    .mount("/", routes![index,lobby, msg, server])
    .mount("/", FileServer::from(relative!("files")))
}