mod dao;
mod minero;
use std::env;

// Para poder pasar por la linea de comandos en que ruta buscar.
fn main() {
    let args: Vec<String> = env::args().collect();
    let ruta = if args.len() == 1 { "." } else { &args[1] };
    let canciones = minero::mina(ruta);
    let _ = dao::dao::conecta_db(&canciones);
}
