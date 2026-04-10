use super::audio;
use super::audio::Cancion;
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
        // Pasamos el archivo especifico, asi funcionan los que buscan los metadatos.
        if es_mp3(&entry) {
            canciones.push(audio::mp3(entry.path()))
        }
    }
    canciones
}
