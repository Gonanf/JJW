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
LobbysDiv.replaceChildren();

let Lobbys = [];

if (localStorage["lobbys"]){
  console.log(JSON.parse(localStorage["lobbys"]));
  console.log(Lobbys);
  for (let i = 0; i < JSON.parse(localStorage["lobbys"]).length; i++){
    console.log("Creando lobby con LocalStorage Lobbys ",JSON.parse(localStorage["lobbys"])[i]);
    CrearLobby(JSON.parse(localStorage["lobbys"])[i]);
  }
  Lobbys = JSON.parse(localStorage["lobbys"]);
}


//Si ya tiene guardado localmente un destino, acceder directamente al menu del chat
if (localStorage["destino"]) {
  Consola.hidden = true;
  Chat.hidden = false;
  Nombre.innerHTML = "Lobby " + localStorage["destino"];
  ConectarseServer();
  RecuperarMensajes();
  if (Lobbys.indexOf(localStorage["destino"]) == -1){
    ActualizarLobbys(localStorage["destino"]);
  }
}

//TODO: Reformular el manejo de lobbys teniendo en cuenta la base de datos

BotonAñadirLobby.addEventListener("click", function(a){
  a.preventDefault();
  FormLobby.hidden = false;
})

FormLobby.addEventListener("submit", function(a){
  a.preventDefault();
  if (!NombreLobby){FormLobby.hidden = true; return}
  console.log("Creando lobby con el form ", NombreLobby.value);
  CrearLobby(NombreLobby.value);
  ActualizarLobbys(NombreLobby.value)
  NombreLobby.value = "";
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
  console.log("Creando lobby con el consola form ",localStorage["destino"])
  CrearLobby(localStorage["destino"]);
  ActualizarLobbys(localStorage["destino"]);
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
    body: new URLSearchParams({ usuario, destino, mensaje}),
  }).then(function (r) {
    if (r.ok) {
      document.getElementById("mensaje").value = "";
    }
  });
  return true;
});

function ActualizarLobbys(nombre){
  if (Lobbys.indexOf(nombre) == -1 && !nombre == ""){
    console.log("Antes: ",Lobbys);
    Lobbys.push(nombre);
    console.log("Despues: ",Lobbys);
    localStorage["lobbys"] = JSON.stringify(Lobbys);
    console.log("Local: ",JSON.parse(localStorage["lobbys"]));
  }
}

function CrearLobby(nombre){
  if (Lobbys.indexOf(nombre) == -1 && !nombre == ""){
    let a = document.createElement("button");
  a.appendChild(document.createTextNode(nombre));
  let b = document.createElement("button");
  b.appendChild(document.createTextNode("-"));
  let div = document.createElement("div");
  div.appendChild(a);
  div.appendChild(b);
  LobbysDiv.appendChild(div);

  a.addEventListener("click", function(a){
    a.preventDefault();
    localStorage["destino"] = nombre;
    RecuperarMensajes();
    Nombre.innerHTML = "Lobby " + localStorage["destino"];
  })

  b.addEventListener("click", function(d){
    d.preventDefault();
    console.log("Antes: ",Lobbys);
    Lobbys.splice(Lobbys.indexOf(nombre),1);
    console.log("Despues: ",Lobbys);
    localStorage["lobbys"] = JSON.stringify(Lobbys);
    console.log("Local: ",JSON.parse(localStorage["lobbys"]));
    console.log(Lobbys.length);
    if (Lobbys.length == 0){
      Consola.hidden = false;
      Chat.hidden = true;
      Nombre.innerHTML = "Lobby";
      localStorage.removeItem("destino");
    }
    else{
      localStorage["destino"] = Lobbys[Lobbys.length - 1];
      console.log("El destino es ", localStorage["destino"]);
      Nombre.innerHTML = "Lobby " + localStorage["destino"];
      RecuperarMensajes();
    }
    LobbysDiv.removeChild(div);
  })
  }
}

//Añade un mensaje en el chat del usuario
function CrearMensaje(usuario, destino, mensaje) {
  let MensajeElemento = document.createElement("p");
  if (usuario != localStorage["nombre"]){
    let NombreUsuario = document.createElement("span")
    NombreUsuario.appendChild(document.createTextNode(usuario + ":"));
    MensajeElemento.appendChild(NombreUsuario);
    MensajeElemento.style.background = "gray";
  }
  else{
    MensajeElemento.style.background = "green";
  }
  MensajeElemento.style.width = "100%";
  MensajeElemento.appendChild(document.createTextNode(mensaje));
  Msg.appendChild(MensajeElemento);
}


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
  const ServerEvents = new EventSource("/server");

  ServerEvents.addEventListener("message", (s) => {
    s.preventDefault();
    let ms = JSON.parse(s.data);
    if (!"usuario" in ms || !"destino" in ms || !"mensaje" in ms) {
      return;
    }
    CrearMensaje(ms.usuario, ms.destino, ms.mensaje);
    return true;
  });
  ServerEvents.addEventListener("open", function (s) {
    console.log("Conectao");
    return true;
  });
  ServerEvents.addEventListener("error", function (s) {
    console.log("error " + s.data);
    setTimeout(() => ConectarseServer(), (() => 1 * 1000)());
    return false;
  });
  
  return true
}
