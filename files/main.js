let forma = document.getElementById("forma");
let nombre = document.getElementById("nombre");
let contraseña = document.getElementById("contraseña");
if (
  localStorage["nombre"] &&
  localStorage["contraseña"] &&
  //TODO: Reformular esto, que use una funcion especialmente para saber si el nombre y contraseña coinciden
  !verificar_usuario(localStorage["nombre"], localStorage["contraseña"])
) {
  document.getElementById("ingresar").textContent =
    "Hola " + localStorage["nombre"] + ", Quieres cambiar de usuario?";
  forma.hidden = true;
  let s = document.createElement("button");
  s.appendChild(document.createTextNode("Si"));
  document.body.appendChild(s);
  let no = document.createElement("button");
  no.appendChild(document.createTextNode("No"));
  document.body.appendChild(no);
  s.onclick = function () {
    localStorage.clear();
    forma.hidden = false;
    s.hidden = true;
    no.hidden = true;
  };
  no.onclick = function () {
    document.location.href = "http://127.0.0.1:8000/lobby";
  };
}
forma.addEventListener("submit", function (a) {
  a.preventDefault();
  if (!nombre.value) {
    alert("El nombre esta vacio");
    return;
  }
  if (!contraseña.value) {
    alert("La contraseña esta vacia");
    return;
  }

  if (verificar_usuario(nombre.value, contraseña.value)) {
    localStorage["nombre"] = nombre.value;
    localStorage["contraseña"] = contraseña.value;
    document.location.href = "http://127.0.0.1:8000/lobby";
  } else {
    alert("Contraseña equivocada (o algo malo paso)");
  }
});

async function verificar_usuario(nombre, contraseña) {
  let lobbys = ["main"];
  let user = await fetch("/registrarse", {
    method: "POST",
    body: new URLSearchParams({ nombre, contraseña, lobbys }),
  })
    .then((response) => response.json())
    .then((data) => {
      console.log(data);
      if (data && data.code == 202) {
        return true;
      }
      return false;
    });
}
