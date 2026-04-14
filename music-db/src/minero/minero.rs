use super::audio;
use super::audio::Cancion;
use std::fs;
use walkdir::{DirEntry, WalkDir};

fn es_mp3(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|x| x.ends_with(".mp3"))
        .unwrap_or(false)
}

pub fn mina(ruta: &str) -> Vec<Cancion> {
    let directorio = WalkDir::new(ruta).into_iter().filter_map(|x| x.ok());
    let mut canciones: Vec<Cancion> = Vec::new();

    for entry in directorio {
        if !entry.file_type().is_file() {
            continue;
        }
        if es_mp3(&entry) {
            match fs::canonicalize(entry.path()) {
                Ok(absolute_path) => canciones.push(audio::mp3(absolute_path.as_path())),
                Err(e) => println!("Error al obtener la ruta: {}", e),
            }
        }
    }
    canciones
}
