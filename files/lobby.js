let Consola = document.getElementById('consola');
let ConsolaForm = document.getElementById('formConsola');
let Destino = document.getElementById('destino');
let Volver = document.getElementById('volver')

let Chat = document.getElementById('chat');
Chat.hidden = true;
let Salir = document.getElementById('salir');
let Nombre = document.getElementById('nombreChat');
let Msg = document.getElementById('mensajes');
let Mensajeador = document.getElementById('meth');

if (localStorage['destino']){
    Consola.hidden = true;
    Chat.hidden = false;
    Nombre.innerHTML = "Lobby " + localStorage['destino'];
    ConectarseServer();
}

Volver.addEventListener('click', function(){
    document.location.href = "http://192.168.0.110:80/";
})

Salir.addEventListener('click', function(){
    Consola.hidden = false;
    Chat.hidden = true;
    Nombre.innerHTML = "Lobby";
})

ConsolaForm.addEventListener('submit', function(a){
    if (!Destino.value){
        alert("No hay un destino");
        a.preventDefault();
    }
    localStorage['destino'] = Destino.value;
    Nombre.innerHTML = "Lobby " + localStorage['destino'];
})

function CrearMensaje(usuario,destino,mensaje){
    if (localStorage['destino'] == destino){
        let MensajeElemento = document.createTextNode(usuario + ":" + mensaje);
        Msg.appendChild(MensajeElemento);
    }
}

function ConectarseServer(){
    if (localStorage['destino']) {
        const ServerEvents = new EventSource("/server");
        ServerEvents.addEventListener('message', (s) => {
            console.log("raw data", JSON.stringify(s.data));
          console.log("decoded data", JSON.stringify(JSON.parse(s.data)));
          let ms = JSON.parse(s.data);
          if (!"usuario" in ms || !"destino" in ms || !"mensaje" in ms){return;}
            CrearMensaje(ms.usuario,ms.destino,ms.mensaje);
        })
        ServerEvents.addEventListener('open', function(s){
            console.log("Conectao");
        })
        ServerEvents.addEventListener('error', function(s){
            console.log("error")
            setTimeout(() => connect(uri), (() => 1 * 1000)());
        })
    }

}

function init(){
    ConectarseServer();

    Mensajeador.addEventListener('submit', function(a){
        a.preventDefault();
        if (!Mensajeador.getElementById('mensaje').value){
            return;
        }
        let Nombre = localStorage['nombre'];
        let Destino = localStorage['destino'];
        let Mensaje = Mensajeador.getElementById('mensaje').value;
        fetch("/mensaje", {
            method: "POST",
            body: new URLSearchParams({Nombre,Destino,Mensaje}),
        }).then(function(r) {
                if (r.ok) Mensajeador.getElementById('mensaje').value = "";

            })
    })}