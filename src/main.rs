use rocket::form::{Form, FromForm};
use rocket::fs::{relative, FileServer};
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::select;
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
    let mut Subs = q.subscribe();
    EventStream! {
        loop {
            let ServM = select! {
                ServM = Subs.recv() => match ServM {
                    Ok(ServM) => ServM,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut t => break,
            };
            
            yield Event::json(&ServM);
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