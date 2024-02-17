use std::env;

use rocket::form::{Form, FromForm};
use rocket::fs::{relative, FileServer};
use rocket::futures::future::ok;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::{fs::NamedFile, serde::{Deserialize, Serialize}, State, Shutdown};
use surrealdb::engine::local::File;
use surrealdb::Surreal;

use surrealdb::engine::local::Db;
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
    mensaje: String,
}

#[post("/mensaje", data = "<ms>")]
async fn msg(ms: Form<Sesion>,q: &State<Sender<Sesion>>, db: &State<Surreal<Db>>){
    let c: Vec<Sesion> = db.create("mensajes")
    .content(Sesion {
        usuario: ms.usuario.to_string(),
        destino: ms.destino.to_string(),
        mensaje: ms.mensaje.to_string(),
    }).await.expect("Error en crear a mesias");
    dbg!(c);
    let _f = q.send(ms.into_inner());
}

/*async fn crear_msg(){
    let c: Vec<Sesion> = db.create("mensajes")
    .content(Sesion {
        usuario: sesion.usuario.to_string(),
        destino: sesion.destino.to_string(),
        mensaje: sesion.mensaje.to_string(),
    }).await.expect("Error en crear a mesias");
    dbg!(c);
    let a: Vec<Sesion> = db.select("usuarios").await.expect("Error en obtner usuarios");
    let b: &String  = &a.get(0).unwrap().usuario;
    println!("{a:?}");
    println!("{b}");
} */

#[post["/restaurar",data = "<sesion>"]]
async fn restaurar_mensajes(sesion: Form<Sesion>, db: &State<Surreal<Db>>) -> Json<Vec<Sesion>>{
    let a: Vec<Sesion> = db.select("mensajes").await.expect("Error en obtner mensajes");
    let mut envio: Vec<Sesion> = Vec::new();
    if !a.is_empty(){
        for s in a{
            if s.destino == sesion.destino {
                envio.push(s);
            }
        }
    }
    Json(envio)
}

//TODO: Mensajes de lobbys con SQL
//TODO: Lobbys viejos eliminan sus mensajes
//TODO: Recuperacion de mensajes de lobbys
#[post("/server", data = "<sesion>")]
async fn server(sesion: Form<Sesion>,q: &State<Sender<Sesion>>, mut t: Shutdown) -> EventStream![]{
    let mut subs = q.subscribe();
    EventStream! {
        loop {
            let serv_m = select! {
                serv_m = subs.recv() => match serv_m {
                    Ok(serv_m) => {
                        println!("Todo bein");
                        if sesion.destino == serv_m.destino{
                            println!("Es tu mensaje");
                            serv_m
                        }
                        else {
                            println!("no es tu mensaje");
                            continue
                        }
                        },
                    Err(RecvError::Closed) => {
                        println!("Error, se corto la conexion");
                        break},
                    Err(RecvError::Lagged(_)) => {
                        println!("Lageo");
                        continue},
                },
                _ = &mut t => {println!("Termino con t"); break},
            };

            yield Event::json(&serv_m);
        }
        println!("Ya no anda activo {}",sesion.usuario);
    }


}




#[launch]
async fn rocket() -> _ {
    let db: Surreal<Db> = Surreal::new::<File>("database").await.expect("No se cargo la base de datos en rocket launch");
    db.use_ns("Chaos").use_db("JJW").await.expect("Error en cambiar de ns y db");
    let c: Vec<Sesion> = db.create("mensajes")
    .content(Sesion {
        usuario: "Miguelito".to_string(),
        destino: "Main".to_string(),
        mensaje: "Prueba".to_string(),
    }).await.expect("Error en crear a mesias");
    dbg!(c);
    rocket::build()
    .manage(channel::<Sesion>(500).0)
    .manage(db)
    .mount("/", routes![index,lobby, msg, server,restaurar_mensajes ])
    .mount("/", FileServer::from(relative!("files")))
}