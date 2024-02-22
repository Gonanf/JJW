let forma = document.getElementById("forma");
let nomb = document.getElementById("nombre");
let contra = document.getElementById("contraseña");
if (document.cookie.indexOf("ID=") != -1) {
  document.getElementById("ingresar").textContent =
    "Hola, no me se tu nombre, quieres cambiar de usuario?";
  forma.hidden = true;
  let s = document.createElement("button");
  s.appendChild(document.createTextNode("Si"));
  document.body.appendChild(s);
  let no = document.createElement("button");
  no.appendChild(document.createTextNode("No"));
  document.body.appendChild(no);
  s.onclick = function () {
    fetch("/eliminar_cookies", {method: "GET"}).then(function(a){
      forma.hidden = false;
      s.hidden = true;
      no.hidden = true;
    })
  };
  no.onclick = function () {
    window.location.href = '/lobby';
  };
}
forma.addEventListener("submit", function (a) {
  a.preventDefault();
  if (!nomb.value) {
    alert("El nombre esta vacio");
    return;
  }
  if (!contra.value) {
    alert("La contraseña esta vacia");
    return;
  }

  console.log("verificascac");
  let lobbys = ["Main"];
  let destino = "Main"
  let nombre = nomb.value;
  let contraseña = contra.value;
  console.log("Iniciando la verificasao");
  fetch("/registrarse", {
    method: "POST",
    body: new URLSearchParams({ nombre, contraseña, lobbys, destino }),
  })
    .then((response) => response.json())
    .then((data) => {
      console.log(data);
      if (data) {
        console.log(data.code);
        console.log(data.code == "Contraseña invalida");
        console.log(data.code == "Usuario inicio sesion");
        if (data.code == "Contraseña invalida"){
          alert("Contraseña invalida");
    console.log("NOVERIFICO");
          
        }
        if(data.code == "Usuario inicio sesion" || data.code == "Usuario creo sesion"){
          window.location.href = '/lobby';
  console.log("VERIFICO");
        }
      }
    });
  

});
