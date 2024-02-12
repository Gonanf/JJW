  let f = document.getElementById('forma'); 
  let n = document.getElementById('nombre'); 
  let i = document.getElementById('ingresar');
  if (localStorage['nombre']){
  document.getElementById('ingresar').textContent = "Hola " + localStorage['nombre'] + ", Quieres cambiar de nombre?";
  f.hidden = true;
  let s = document.createElement("button"); s.setAttribute('id','Si');
  s.appendChild(document.createTextNode("Si")); document.body.appendChild(s);
  let no = document.createElement("button"); no.setAttribute('id','No');
  no.appendChild(document.createTextNode("No")); document.body.appendChild(no);
  s.onclick = function(){
    localStorage.clear();
    window.location.reload(); }
  no.onclick = function(){ document.location.href =
  "http://192.168.0.110:80/lobby"; } } f.addEventListener('submit', function(a){
  if (!n.value){ alert("El nombre esta vacio"); a.preventDefault(); }
  localStorage['nombre'] = n.value; })