use std::any::{ Any, TypeId };
use std::env;
use std::sync::Condvar;
use rocket::form::{ Form, FromForm };
use rocket::fs::{ relative, FileServer };
use rocket::http::Status;
use rocket::response::stream::{ EventStream, Event };
use rocket::serde::json::{ to_string, Json };
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{ channel, Sender, error::RecvError };
use rocket::{ fs::NamedFile, serde::{ Deserialize, Serialize }, State, Shutdown };
use surrealdb::engine::local::File;
use surrealdb::sql::statements::ContinueStatement;
use surrealdb::sql::{ Thing };
use surrealdb::Surreal;
use rocket::http::{Cookie, CookieJar};
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
    destino: String,
}

//TODO: GET de obtener lobbys del usuario
//TODO: POST de añadir Lobbys al usuario
//TODO: POST de eliminar Lobbys al usuario

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RespuestaId {
    code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Record {
    id: Thing,
}
#[post("/registrarse", data = "<user>")]
async fn registrarse(user: Form<User>, db: &State<Surreal<Db>>, galleta: &CookieJar<'_>) -> Json<RespuestaId> {
    let respuesta: RespuestaId;
    let usuarios: Vec<User> = db.select("usuarios").await.expect("error obteniendo usuarios");
    let mut nombre_coincide = false;
    let mut contraseña_coincide = false;
    let mut posicion_usuario = 0;
    println!("en busqueda de usuarios");
    dbg!(&usuarios);
    for (i, v) in usuarios.iter().enumerate() {
        if v.nombre == user.nombre {
            nombre_coincide = true;
        }
        if v.contraseña == user.contraseña {
            contraseña_coincide = true;
        }
        if v == &user.clone() {
            nombre_coincide = true;
            contraseña_coincide = true;
            posicion_usuario = i;
            break;
        }
        println!("iterando usuarios");
    }
    if nombre_coincide && !contraseña_coincide {
        println!("contra incorrecta");
        respuesta = RespuestaId {
            code: "Contraseña invalida".to_string(),
        };
        return Json(respuesta);
    } else if nombre_coincide && contraseña_coincide {
        //esto es ineficiente, pero lo mas facil
        println!("usuario inicio sesion");
        let id_user: Vec<Record> = db.select("usuarios").await.expect("Error obteniendo records");
        respuesta = RespuestaId {
            code: "Usuario inicio sesion".to_string(),
        };
        println!("añade galletita");
        galleta.add_private(("ID", id_user[posicion_usuario].id.to_string()));
        return Json(respuesta);
    } else {
        println!("usuario creo la sesion");
        let mut temp: Vec<String> = Vec::new();
            temp.push("Main".to_string());
            let usuario: Vec<Record> = db
                .create("usuarios")
                .content(User {
                    nombre: user.nombre.to_string(),
                    contraseña: user.contraseña.to_string(),
                    lobbys: temp,
                    destino: "Main".to_string(),
                }).await
                .expect("error crearndo usuario");
        galleta.add_private(("ID", usuario[0].id.to_string()));
            respuesta = RespuestaId {
                code: "Usuario creo sesion".to_string(),
            };
            return Json(respuesta);
    }
}

#[post("/mensaje", data = "<ms>")]
async fn msg(ms: Form<Sesion>, q: &State<Sender<Sesion>>, db: &State<Surreal<Db>>, galleta: &CookieJar<'_>) {
    let user_data: Option<Cookie<'_>> = galleta.get_private("ID");
    let sd: Cookie<'_> = user_data.expect("Q paso master");
    let usuario: User = db.select(("usuarios", sd.value())).await.expect("Error obteniendo el usuario por ID").expect("wa");
    dbg!(&usuario);
    dbg!(sd);

    let contador: Vec<Contador> = db.select("contador").await.expect("error obteniendo contador");
    let id = contador[0].id_c;
    let _mensaje: Option<Sesion> = db
        .create(("mensajes", id))
        .content(Sesion {
            usuario: usuario.nombre.to_string(),
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

#[get["/restaurar"]]
async fn restaurar_mensajes(db: &State<Surreal<Db>>, galleta: &CookieJar<'_>) -> Json<Vec<Sesion>> {
    println!("AAAAAAAAAAAAAAAAA");
    let user_data: Option<Cookie<'_>> = galleta.get_private("ID");
    let sd: Cookie<'_> = user_data.expect("Q paso master");
    let usuario: User = db.select(("usuarios", sd.value())).await.expect("Error obteniendo el usuario por ID").expect("wa");
    dbg!(&usuario);
    dbg!(sd);
        let mut envio: Vec<Sesion> = Vec::new();

        let a: Vec<Sesion> = db.select("mensajes").await.expect("Error en obtner mensajes");
        if !a.is_empty() {
            for s in a {
                if s.destino == usuario.destino {
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
async fn server(q: &State<Sender<Sesion>>, mut t: Shutdown, db: &State<Surreal<Db>>, galleta: &CookieJar<'_>) -> EventStream![] {
    let user_data: Option<Cookie<'_>> = galleta.get_private("ID");
    let sd: Cookie<'_> = user_data.expect("Q paso master");
    let usuario: User = db.select(("usuarios", sd.value())).await.expect("Error obteniendo el usuario por ID").expect("wa");
    dbg!(&usuario);
    dbg!(sd);
    let mut subs = q.subscribe();
    EventStream! {
        loop {
            let serv_m =
                select! {
                serv_m = subs.recv() => match serv_m {
                    Ok(serv_m) => {
                        println!("Todo bein");
                        if serv_m.destino == usuario.destino{serv_m}
                        else{continue}
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
    }
    rocket
        ::build()
        .manage(channel::<Sesion>(1023).0)
        .manage(db)
        .mount("/", routes![index, lobby, msg, server, restaurar_mensajes, registrarse])
        .mount("/", FileServer::from(relative!("files")))
}
