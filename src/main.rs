use std::env;
use std::sync::Condvar;
use rocket::form::{ Form, FromForm };
use rocket::fs::{ relative, FileServer };
use rocket::http::Status;
use rocket::response::stream::{ EventStream, Event };
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{ channel, Sender, error::RecvError };
use rocket::{ fs::NamedFile, serde::{ Deserialize, Serialize }, State, Shutdown };
use surrealdb::engine::local::File;
use surrealdb::Surreal;

use surrealdb::engine::local::Db;
#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("files/html/main.html").await.ok()
}

#[get("/lobby")]
async fn lobby() -> Option<NamedFile> {
    NamedFile::open("files/html/lobby.html").await.ok()
}

#[derive(Debug, FromForm, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Sesion {
    usuario: String,
    destino: String,
    mensaje: String,
}

#[derive(Debug, FromForm, Clone, Serialize, Deserialize, PartialEq)]
#[serde(crate = "rocket::serde")]
struct User {
    nombre: String,
    contraseña: String,
    lobbys: Vec<String>,
}

//TODO: GET de obtener lobbys del usuario
//TODO: POST de añadir Lobbys al usuario
//TODO: POST de eliminar Lobbys al usuario

#[post("/registrarse", data = "<user>")]
async fn registrarse(user: Form<User>, db: &State<Surreal<Db>>) -> Status {
    let usuarios: Vec<User> = db.select("usuarios").await.expect("error obteniendo usuarios");

    fn usuario_valido(US: &Vec<User>, usuario: &User) -> i8 {
        let mut nombre_coincide = false;
        let mut contraseña_coincide = false;
        for usuario_db in US {
            if usuario_db.nombre == usuario.nombre {
                nombre_coincide = true;
            }
            if usuario_db.contraseña == usuario.contraseña {
                contraseña_coincide = true;
            }
        }
        if nombre_coincide && contraseña_coincide {
            return 0;
        } else if nombre_coincide && !contraseña_coincide {
            return 1;
        } else {
            return 2;
        }
    }

    match usuario_valido(&usuarios, &user) {
        0 => {
        //TODO: Reformular esto, q use un status diferente

            return Status::Accepted;
        }
        1 => {
            return Status::NotAcceptable;
        }
        2 => {
            let mut temp: Vec<String> = Vec::new();
            temp.push("Main".to_string());
            let contador: Vec<Contador> = db
                .select("contador").await
                .expect("Error obteniendo contador usuarios");
            let id = contador[1].id_c;
            let _usuario: Option<User> = db
                .create(("usuarios", id))
                .content(User {
                    nombre: user.nombre.to_string(),
                    contraseña: user.contraseña.to_string(),
                    lobbys: temp,
                }).await
                .expect("error crearndo usuario");
            let _contador: Option<Contador> = db
                .update(("contador", 1))
                .content(Contador { id_c: id + 1 }).await
                .expect("error en actualizar contador");
            return Status::Accepted;
        }
        _ => {
            return Status::NotFound;
        }
    }
}

#[post("/mensaje", data = "<ms>")]
async fn msg(ms: Form<Sesion>, q: &State<Sender<Sesion>>, db: &State<Surreal<Db>>) {
    let contador: Vec<Contador> = db.select("contador").await.expect("error obteniendo contador");
    let id = contador[0].id_c;
    let _mensaje: Option<Sesion> = db
        .create(("mensajes", id))
        .content(Sesion {
            usuario: ms.usuario.to_string(),
            destino: ms.destino.to_string(),
            mensaje: ms.mensaje.to_string(),
        }).await
        .expect("Error en crear a mesias");
    let s: Vec<Sesion> = db.select("mensajes").await.expect("msg");
    for v in s {
        println!("Mensaje: {}", v.mensaje);
    }
    let _contador: Option<Contador> = db
        .update(("contador", 0))
        .content(Contador { id_c: id + 1 }).await
        .expect("error en actualizar contador");
    let _f = q.send(ms.into_inner());
}

#[post["/restaurar",data = "<sesion>"]]
async fn restaurar_mensajes(sesion: Form<Sesion>, db: &State<Surreal<Db>>) -> Json<Vec<Sesion>> {
    let a: Vec<Sesion> = db.select("mensajes").await.expect("Error en obtner mensajes");
    let mut envio: Vec<Sesion> = Vec::new();
    if !a.is_empty() {
        for s in a {
            if s.destino == sesion.destino {
                envio.push(s);
            }
        }
    }
    dbg!(&envio);
    Json(envio)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Contador {
    id_c: i64,
}

#[get("/server")]
async fn server(q: &State<Sender<Sesion>>, mut t: Shutdown) -> EventStream![] {
    let mut subs = q.subscribe();
    EventStream! {
        loop {
            let serv_m =
                select! {
                serv_m = subs.recv() => match serv_m {
                    Ok(serv_m) => {
                        println!("Todo bein");
                        serv_m
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
        println!("Ya no anda activo");
    }
}

#[launch]
async fn rocket() -> _ {
    let db: Surreal<Db> = Surreal::new::<File>("database").await.expect(
        "No se cargo la base de datos en rocket launch"
    );
    db.use_ns("Chaos").use_db("JJW").await.expect("Error en cambiar de ns y db");
    let search_counter: Vec<Contador> = db.select("contador").await.expect("Errorsito uwu");
    if search_counter.len() == 0 {
        let _c: Option<Contador> = db
            .create(("contador", 0))
            .content(Contador { id_c: 0 }).await
            .expect("Errorsito contador");
        let _c: Option<Contador> = db
            .create(("contador", 1))
            .content(Contador { id_c: 0 }).await
            .expect("Errorsito contador");
    } else if search_counter.len() != 2 {
        let _c: Option<Contador> = db
            .create(("contador", 1))
            .content(Contador { id_c: 0 }).await
            .expect("Errorsito contador");
    }
    rocket
        ::build()
        .manage(channel::<Sesion>(500).0)
        .manage(db)
        .mount("/", routes![index, lobby, msg, server, restaurar_mensajes, registrarse])
        .mount("/", FileServer::from(relative!("files")))
}
