let Consola = document.getElementById('consola');
let ConsolaForm = document.getElementById('formConsola');
let Destino = document.getElementById('destino');
let Volver = document.getElementById('volver')

let Chat = document.getElementById('chat');
Chat.hidden = true;
let Salir = document.getElementById('salir');
let Nombre = document.getElementById('nombreChat');

if (localStorage['destino']){
    Consola.hidden = true;
    Chat.hidden = false;
    Nombre.innerHTML = "Lobby " + localStorage['destino'];
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