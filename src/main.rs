use std::env;
use std::ops::Add;
use uuid::Uuid;
use rocket::form::{ Form, FromForm };
use rocket::fs::{ relative, FileServer };
use rocket::response::stream::{ EventStream, Event };
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{ channel, Sender, error::RecvError };
use rocket::{ fs::NamedFile, serde::{ Deserialize, Serialize }, State, Shutdown };
use surrealdb::engine::local::File;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use rocket::http::{ Cookie, CookieJar, SameSite };
use surrealdb::engine::local::Db;
use unicode_segmentation::UnicodeSegmentation;
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

#[derive(Debug, FromForm, Clone, Serialize, Deserialize)]
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
async fn registrarse(
    user: Form<User>,
    db: &State<Surreal<Db>>,
    galleta: &CookieJar<'_>
) -> Json<RespuestaId> {
    let respuesta: RespuestaId;
    let usuarios: Vec<User> = db.select("usuarios").await.expect("error obteniendo usuarios");
    let mut nombre_coincide = false;
    let mut contraseña_coincide = false;
    let mut nombre_usuario: String = String::new();
    for v in usuarios.iter() {
        if v.nombre == user.nombre {
            nombre_coincide = true;
        }
        if v.contraseña == user.contraseña {
            contraseña_coincide = true;
        }
        if nombre_coincide && contraseña_coincide {
            nombre_usuario = v.nombre.clone();
            break;
        }
    }
    if nombre_coincide && !contraseña_coincide {
        respuesta = RespuestaId {
            code: "Contraseña invalida".to_string(),
        };
        return Json(respuesta);
    } else if nombre_coincide && contraseña_coincide {
        //esto es ineficiente, pero lo mas facil
        let mut surrealql: surrealdb::Response = db
            .query("SELECT * FROM usuarios WHERE nombre = $nom")
            .bind(("nom", nombre_usuario)).await
            .expect("Error obteniendo usuario ID");
        let id_usuario: Vec<Record> = surrealql.take(0).unwrap();
        respuesta = RespuestaId {
            code: "Usuario inicio sesion".to_string(),
        };
        let c = Cookie::build(("ID", id_usuario[0].id.to_string()))
            .secure(false)
            .same_site(SameSite::Lax);
        galleta.add_private(c);
        return Json(respuesta);
    } else {
        let id: Uuid = Uuid::now_v7();
        let mut temp: Vec<String> = Vec::new();
        temp.push("Main".to_string());
        let usuario: Record = db
            .create(("usuarios", id.to_string()))
            .content(User {
                nombre: user.nombre.to_string(),
                contraseña: user.contraseña.to_string(),
                lobbys: temp,
                destino: "Main".to_string(),
            }).await
            .expect("error crearndo usuario")
            .unwrap();
        let c = Cookie::build(("ID", usuario.id.to_string()))
            .secure(false)
            .same_site(SameSite::Lax);
        galleta.add_private(c);
        respuesta = RespuestaId {
            code: "Usuario creo sesion".to_string(),
        };
        return Json(respuesta);
    }
}

fn de_cookies_a_uuid(id: Cookie<'_>) -> String {
    let id_a: Vec<&str> = id.value().split(":").collect();
    let id_uuid: String = id_a[1].to_string();
    let mut id_final = String::new();
    for (i, c) in id_uuid.chars().enumerate() {
        if i != 0 && i != id_uuid.graphemes(true).count() - 1 {
            id_final = id_final.clone().add(&c.to_string());
        }
    }
    return id_final;
}

#[get("/obtener_datos")]
async fn obtener_datos(db: &State<Surreal<Db>>, galleta: &CookieJar<'_>) -> Json<User> {
    let user_data: Cookie<'_> = galleta.get_private("ID").expect("q paso master");
    let id_final = de_cookies_a_uuid(user_data);
    let usuario: Option<User> = db.select(("usuarios", id_final)).await.unwrap();
    let mut f: User = usuario.unwrap().clone();
    f.contraseña = "*****".to_string();
    return Json(f);
}

#[post("/añadir_lobby", data = "<lobby>")]
async fn añadir_lobby(lobby: Form<RespuestaId>, db: &State<Surreal<Db>>, galleta: &CookieJar<'_>) {
    let user_data: Cookie<'_> = galleta.get_private("ID").expect("q paso master");
    let id_final = de_cookies_a_uuid(user_data);
    let mut usuario: User = db
        .select(("usuarios", id_final.clone())).await
        .expect("Error obteniendo el usuario por ID")
        .expect("wa");
    usuario.lobbys.push(lobby.code.clone());
    let _actualizado: Option<User> = db
        .update(("usuarios", id_final.clone()))
        .content(User {
            nombre: usuario.nombre,
            contraseña: usuario.contraseña,
            lobbys: usuario.lobbys,
            destino: lobby.code.clone(),
        }).await
        .expect("Error actualizando lobbys");
}

#[post("/cambiar_lobby", data = "<lobby>")]
async fn cambiar_lobby(lobby: Form<RespuestaId>, db: &State<Surreal<Db>>, galleta: &CookieJar<'_>) {
    let user_data: Cookie<'_> = galleta.get_private("ID").expect("q paso master");
    let id_final = de_cookies_a_uuid(user_data);
    let usuario: User = db
        .select(("usuarios", id_final.clone())).await
        .expect("Error obteniendo el usuario por ID")
        .expect("wa");
    let _actualizado: Option<User> = db
        .update(("usuarios", id_final.clone()))
        .content(User {
            nombre: usuario.nombre,
            contraseña: usuario.contraseña,
            lobbys: usuario.lobbys,
            destino: lobby.code.clone(),
        }).await
        .expect("Error actualizando lobbys");
}

#[post("/eliminar_lobby", data = "<lobby>")]
async fn eliminar_lobby(
    lobby: Form<RespuestaId>,
    db: &State<Surreal<Db>>,
    galleta: &CookieJar<'_>
) {
    let user_data: Cookie<'_> = galleta.get_private("ID").expect("q paso master");
    let id_final = de_cookies_a_uuid(user_data);
    let mut usuario: User = db
        .select(("usuarios", id_final.clone())).await
        .expect("Error obteniendo el usuario por ID")
        .expect("wa");
    for (i, v) in usuario.lobbys.iter().enumerate() {
        if v == &lobby.code {
            usuario.lobbys.remove(i);
            break;
        }
    }
    let lobby_nuevo: Option<&String> = usuario.lobbys.last();
    let ln;
    if lobby_nuevo == None {
        ln = "Main".to_string();
    } else {
        ln = lobby_nuevo.expect("aw").clone();
    }
    let _actualizado: Option<User> = db
        .update(("usuarios", id_final.clone()))
        .content(User {
            nombre: usuario.nombre,
            contraseña: usuario.contraseña,
            lobbys: usuario.lobbys,
            destino: ln,
        }).await
        .expect("Error actualizando lobbys");
}

#[get("/eliminar_cookies")]
async fn eliminar_cookies(galleta: &CookieJar<'_>) {
    galleta.remove_private("ID");
}

#[post("/mensaje", data = "<ms>")]
async fn msg(
    mut ms: Form<Sesion>,
    q: &State<Sender<Sesion>>,
    db: &State<Surreal<Db>>,
    galleta: &CookieJar<'_>
) {
    let user_data: Cookie<'_> = galleta.get_private("ID").expect("Q paso master");
    let id_final = de_cookies_a_uuid(user_data);
    let usuario: User = db
        .select(("usuarios", id_final.clone())).await
        .expect("Error obteniendo el usuario por ID")
        .expect("wa");
    let contador: Vec<Contador> = db.select("contador").await.expect("error obteniendo contador");
    let id = contador[0].id_c;
    let _mensaje: Option<Sesion> = db
        .create(("mensajes", id))
        .content(Sesion {
            usuario: usuario.nombre.to_string(),
            destino: usuario.destino.to_string(),
            mensaje: ms.mensaje.clone(),
        }).await
        .expect("Error en crear a mesias");
    ms = (Sesion {
        usuario: usuario.nombre.to_string(),
        destino: usuario.destino.to_string(),
        mensaje: ms.mensaje.clone(),
    }).into();
    let _contador: Option<Contador> = db
        .update(("contador", 0))
        .content(Contador { id_c: id + 1 }).await
        .expect("error en actualizar contador");
    let _f = q.send(ms.into_inner());
}

#[get["/restaurar"]]
async fn restaurar_mensajes(db: &State<Surreal<Db>>, galleta: &CookieJar<'_>) -> Json<Vec<Sesion>> {
    let user_data: Cookie<'_> = galleta.get_private("ID").expect("Q paso master");
    let id_final = de_cookies_a_uuid(user_data);
    let usuario: User = db
        .select(("usuarios", id_final)).await
        .expect("Error obteniendo el usuario por ID")
        .expect("wa");
    let mut a = db
        .query("SELECT * FROM mensajes WHERE destino = $usuario ")
        .bind(("usuario", usuario.destino)).await
        .expect("Error obteniendo mensajes restaurar");
    //let a: Vec<Sesion> = db.select("mensajes").await.expect("Error en obtner mensajes");
    let b: Vec<Sesion> = a.take(0).unwrap();
    Json(b)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Contador {
    id_c: i64,
}

#[get("/server")]
async fn server<'a>(
    q: &'a State<Sender<Sesion>>,
    mut t: Shutdown,
    db: &'a State<Surreal<Db>>,
    galleta: &'a CookieJar<'a>
) -> EventStream![Event + 'a] {
    let user_data: Cookie<'_> = galleta.get_private("ID").expect("Q paso master");
    let id_final = de_cookies_a_uuid(user_data);
    let mut subs = q.subscribe();
    EventStream! {
        loop {
            let serv_m =
                select! {
                serv_m = subs.recv() => match serv_m {
                    Ok(serv_m) => {
                        let usuario: User = db.select(("usuarios", id_final.clone())).await.unwrap().unwrap();
                        if serv_m.destino == usuario.destino { serv_m}
                        else{continue}
                        },
                    Err(RecvError::Closed) => {
                        break},
                    Err(RecvError::Lagged(_)) => {
                        continue},
                },
                _ = &mut t => { break},
            };
            yield Event::json(&serv_m);
        }
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
        .mount(
            "/",
            routes![
                index,
                lobby,
                msg,
                server,
                restaurar_mensajes,
                registrarse,
                obtener_datos,
                añadir_lobby,
                eliminar_lobby,
                eliminar_cookies,
                cambiar_lobby
            ]
        )
        .mount("/", FileServer::from(relative!("files")))
}
