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

let nombre;
let destino;

AñadirLobbys();

function AñadirLobbys() {
  fetch("/obtener_datos", { method: "GET" })
    .then((response) => response.json())
    .then((data) => {
      if (data) {
        console.log(data);
        console.log("Lobbys:");
        console.log(data.lobbys);
        console.log(data.lobbys.length);
        if (data.lobbys.length > 0) {
          for (let i = 0; i < data.lobbys.length; i++) {
            console.log("Creando lobby con Cookies Lobbys ", data.lobbys[i]);
            CrearLobby(data.lobbys[i]);
          }
          Lobbys = data.lobbys;
        }
        console.log("Skipeando");
        Consola.hidden = true;
        Chat.hidden = false;
        Nombre.innerHTML = "Lobby " + data.destino;
        ConectarseServer();
        RecuperarMensajes();
        if (Lobbys.indexOf(data.destino) == -1) {
          ActualizarLobbys(data.destino);
        }
        console.log("Nombre: " + data.nombre + " Destino: " + data.destino);
        Nombre.innerHTML = "Lobby " + data.destino;
        nombre = data.nombre;
        destino = data.destino;
      }
    });
}

BotonAñadirLobby.addEventListener("click", function (a) {
  a.preventDefault();
  FormLobby.hidden = false;
});

FormLobby.addEventListener("submit", function (a) {
  a.preventDefault();
  if (!NombreLobby) {
    FormLobby.hidden = true;
    return;
  }
  console.log("Creando lobby con el form ", NombreLobby.value);
  CrearLobby(NombreLobby.value);
  ActualizarLobbys(NombreLobby.value);
  Nombre.innerHTML = "Lobby " + NombreLobby.value;
  RecuperarMensajes();
  NombreLobby.value = "";
  FormLobby.hidden = true;
});

//Boton para volver al menu de sesion
Volver.addEventListener("click", function () {
  window.location.href = "/";
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
  let code = Destino.value;
  fetch("/cambiar_lobby", {
    method: "POST",
    body: new URLSearchParams({ code }),
  }).then(() => {
    Consola.hidden = true;
    Chat.hidden = false;
    Nombre.innerHTML = "Lobby " + Destino.value;
    destino = code;
    ConectarseServer();
    RecuperarMensajes();
    console.log("Creando lobby con el consola form ", Destino.value);
    CrearLobby(Destino.value);
    ActualizarLobbys(Destino.value);
    return true;
  });
});

//funcion del boton que envia el mensaje, obtiene todos los datos y los manda para el back end
Mensajeador.addEventListener("click", function (a) {
  a.preventDefault();
  if (!document.getElementById("mensaje").value) {
    return;
  }
  let mensaje = document.getElementById("mensaje").value;
  let usuario = "";
  fetch("/mensaje", {
    method: "POST",
    body: new URLSearchParams({ usuario, destino, mensaje }),
  }).then(function (r) {
    if (r.ok) {
      document.getElementById("mensaje").value = "";
    }
  });
  return true;
});

function ActualizarLobbys(nombre) {
  if (Lobbys.indexOf(nombre) == -1 && !nombre == "") {
    let code = nombre;
    fetch("/añadir_lobby", {
      method: "POST",
      body: new URLSearchParams({ code }),
    });
    Lobbys.push(nombre);
  }
}

function CrearLobby(nombre) {
  if (Lobbys.indexOf(nombre) == -1 && !nombre == "") {
    let a = document.createElement("button");
    a.appendChild(document.createTextNode(nombre));
    let b = document.createElement("button");
    b.appendChild(document.createTextNode("-"));
    let div = document.createElement("div");
    div.appendChild(a);
    div.appendChild(b);
    LobbysDiv.appendChild(div);

    a.addEventListener("click", function (a) {
      a.preventDefault();
      let code = nombre;
      fetch("/cambiar_lobby", {
        method: "POST",
        body: new URLSearchParams({ code }),
      }).then(RecuperarMensajes());
      Nombre.innerHTML = "Lobby " + nombre;
      destino = nombre;
    });

    b.addEventListener("click", function (d) {
      d.preventDefault();
      let code = nombre;
      console.log("Antes: ", Lobbys);
      Lobbys.splice(Lobbys.indexOf(nombre), 1);
      console.log("Despues: ", Lobbys);
      fetch("/eliminar_lobby", {
        method: "POST",
        body: new URLSearchParams({ code }),
      });

      if (Lobbys.length == 0) {
        Consola.hidden = false;
        Chat.hidden = true;
        Nombre.innerHTML = "Lobby";
      } else {
        code = Lobbys[Lobbys.length - 1];
        fetch("/cambiar_lobby", {
          method: "POST",
          body: new URLSearchParams({ code }),
        }).then(RecuperarMensajes());
        destino = Lobbys[Lobbys.length - 1];
        Nombre.innerHTML = "Lobby " + Lobbys[Lobbys.length - 1];
      }
      LobbysDiv.removeChild(div);
    });
  }
}

//Añade un mensaje en el chat del usuario
function CrearMensaje(usuario, mensaje) {
  let MensajeElemento = document.createElement("p");
  if (usuario != nombre) {
    let NombreUsuario = document.createElement("span");
    NombreUsuario.appendChild(document.createTextNode(usuario + ":"));
    MensajeElemento.appendChild(NombreUsuario);
    MensajeElemento.style.background = "gray";
  } else {
    MensajeElemento.style.background = "green";
  }
  MensajeElemento.style.width = "100%";
  MensajeElemento.appendChild(document.createTextNode(mensaje));
  Msg.appendChild(MensajeElemento);
}

function RecuperarMensajes() {
  Msg.replaceChildren();
  let db = fetch("/restaurar", { method: "GET" })
    .then((response) => response.json())
    .then((data) => {
      console.log(data.length);
      if (data && data.length > 0) {
        for (let i = 0; i < data.length; i++) {
          CrearMensaje(data[i].usuario, data[i].mensaje);
          console.log(data[i]);
        }
      }
    });
}

function ConectarseServer() {
  const ServerEvents = new EventSource("/server");
  ServerEvents.addEventListener("message", (s) => {
    s.preventDefault();
    let ms = JSON.parse(s.data);
    if (!"usuario" in ms || !"destino" in ms || !"mensaje" in ms) {
      return;
    }
    CrearMensaje(ms.usuario, ms.mensaje);
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

  return true;
}