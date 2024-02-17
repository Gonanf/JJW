//Menu de seleccion / creacion de sala (todo es una mentira)
let Consola = document.getElementById("consola");
let ConsolaForm = document.getElementById("formConsola");
let Destino = document.getElementById("destino");
let Volver = document.getElementById("volver");

//Actividad
let activo = true;

//elementos del chat
let Chat = document.getElementById("chat");
Chat.hidden = true;
let Salir = document.getElementById("salir");
let Nombre = document.getElementById("nombreChat");
let Msg = document.getElementById("mensajes");
let Mensajeador = document.getElementById("meth");

let FormLobby = document.getElementById("form_lobby");
FormLobby.hidden = true;
let BotonAñadirLobby = document.getElementById("añadir");
let NombreLobby = document.getElementById("nombre_lobby");
let LobbysDiv = document.getElementById("lobbys");

let Lobbys = ["Main"];

if (localStorage["lobbys"]){
  console.log(JSON.parse(localStorage["lobbys"]));
  console.log(Lobbys);
  Lobbys = JSON.parse(localStorage["lobbys"]);
  for (let i = 0; i < Lobbys.length; i++){
    CrearLobby(Lobbys[i]);
  }
}

//Si ya tiene guardado localmente un destino, acceder directamente al menu del chat
if (localStorage["destino"]) {
  Consola.hidden = true;
  Chat.hidden = false;
  Nombre.innerHTML = "Lobby " + localStorage["destino"];
  ConectarseServer();
  RecuperarMensajes();
  if (Lobbys.indexOf(localStorage["destino"]) == -1){
    Lobbys.push(localStorage["destino"]);
    localStorage["lobbys"] = JSON.stringify(Lobbys);
  }
}



BotonAñadirLobby.addEventListener("click", function(a){
  a.preventDefault();
  FormLobby.hidden = false;
})

FormLobby.addEventListener("submit", function(a){
  a.preventDefault();
  if (!NombreLobby){FormLobby.hidden = true; return}
  CrearLobby(NombreLobby.value);
  Lobbys.push(NombreLobby.value);
  localStorage["lobbys"] = JSON.stringify(Lobbys);
  FormLobby.hidden = true; 
})

//Boton para volver al menu de sesion
Volver.addEventListener("click", function () {
  document.location.href = "http://127.0.0.1:8000/";
});

//Boton para volver al menu de salas
Salir.addEventListener("click", function () {
  Consola.hidden = false;
  Chat.hidden = true;
  Nombre.innerHTML = "Lobby";
});

//Menu para elegir la sala, se guarda localmente esta decision
ConsolaForm.addEventListener("click", function (a) {
  a.preventDefault();
  if (!Destino.value) {
    alert("No hay un destino");
    return false;
  }
  localStorage["destino"] = Destino.value;
  Consola.hidden = true;
  Chat.hidden = false;
  Nombre.innerHTML = "Lobby " + localStorage["destino"];
  ConectarseServer();
  RecuperarMensajes();
  Lobbys.push(localStorage["destino"]);
  localStorage["lobbys"] = JSON.stringify(Lobbys);
  CrearLobby("Main");
  CrearLobby(localStorage["destino"]);
  return true;
});

//Codigo robao, funcion del boton que envia el mensaje, obtiene todos los datos y los manda para el back end
//PT: No funcionaba por k no coloque las variables con el nombre correcto de los elementos en el struct Mensajes
Mensajeador.addEventListener("click", function (a) {
  a.preventDefault();
  if (!document.getElementById("mensaje").value) {
    return;
  }
  let usuario = localStorage["nombre"];
  let destino = localStorage["destino"];
  let mensaje = document.getElementById("mensaje").value;
  fetch("/mensaje", {
    method: "POST",
    body: new URLSearchParams({ usuario, destino, mensaje, activo }),
  }).then(function (r) {
    if (r.ok) {
      document.getElementById("mensaje").value = "";
    }
  });
  return true;
});

function CrearLobby(nombre){
  let a = document.createElement("button");
  a.appendChild(document.createTextNode(nombre));
  let b = document.createElement("button");
  b.appendChild(document.createTextNode("-"));

  a.appendChild(b);
  LobbysDiv.appendChild(a);

  a.addEventListener("click", function(a){
    a.preventDefault();
    localStorage["destino"] = nombre;
    RecuperarMensajes();
  })

  b.addEventListener("click", function(d){
    d.preventDefault();
    Lobbys.pop(nombre);
    localStorage["lobbys"] = JSON.stringify(Lobbys);
    console.log(Lobbys.length);
    if (Lobbys.length == 0){
      Consola.hidden = false;
      Chat.hidden = true;
      Nombre.innerHTML = "Lobby";
    }
    else{
      localStorage["destino"] = Lobbys[Lobbys.length - 1];
    }
    LobbysDiv.removeChild(a);
  })
}

//Añade un mensaje en el chat del usuario
function CrearMensaje(usuario, destino, mensaje) {
    let MensajeElemento = document.createElement("p");
    let NombreUsuario = document.createElement("span")
    NombreUsuario.appendChild(document.createTextNode(usuario + ":"));
    MensajeElemento.appendChild(NombreUsuario);
    MensajeElemento.appendChild(document.createTextNode(mensaje));
    Msg.appendChild(MensajeElemento);
}

//TODO: Diferenciacion entre mensaje propio y de otro
//TODO: Mensaje de entrar/salir de lobby
//TODO: Multiples lobbys

async function RecuperarMensajes(){
  Msg.replaceChildren();
  let usuario = localStorage["nombre"];
  let destino = localStorage["destino"];
  let mensaje = "nashe";

  let db = await fetch("/restaurar",
  {method: "POST",
  body: new URLSearchParams({ usuario, destino, mensaje, activo })
  }).then(response => response.json())
  .then(data => {
    console.log(data.length)
    if (data && data.length > 0){
      for (let i = 0; i < data.length; i++){
        CrearMensaje(data[i].usuario,data[i].destino,data[i].mensaje);
        console.log(data[i]);
      }
    }

  });


}

async function ConectarseServer() {
  let usuario = localStorage["nombre"];
  let destino = localStorage["destino"];
  let mensaje = "nashe";

  let server = await fetch("/server",
  {method: "POST",
  body: new URLSearchParams({ usuario, destino, mensaje, activo })
  });

  const r = server.body.pipeThrough(new TextDecoderStream()).getReader()
  while (true) {
    if(!server.ok){
      console.log("no conectao {}",v);
      setTimeout(() => ConectarseServer(), (() => 1 * 1000)());
    }
    else{
      const {value, done} = await r.read();
      if (done) break;
      if (!value.includes("usuario") || !value.includes("destino") || !value.includes("mensaje") ) {
        console.log("Errorsito, no esta formateado");
      }
      else{
        console.log(value.toString());
        let parse = JSON.parse(value.replace("data:",""));
        console.log(parse);
        CrearMensaje(parse.usuario,parse.destino,parse.mensaje);
      }
    }
  }
  return true
}
