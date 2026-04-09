use walkdir::{WalkDir, DirEntry};
use super::audio;

fn es_mp3(entry: &DirEntry) -> bool {
	entry.file_name().to_str()
	.map(|x| x.ends_with(".mp3"))
	.unwrap_or(false)
}

pub fn mina(ruta: &str) -> Result<(), Box<dyn std::error::Error>> {
	let directorio = WalkDir::new(ruta).into_iter().filter_map(|x| x.ok());

	for entry in directorio {

		if !entry.file_type().is_file() {
			continue;
		}
		// Pasamos el archivo especifico, asi funcionan los que buscan los metadatos.
		if es_mp3(&entry) {
			let _ = audio::mp3(entry.path());
		}
	}
	Ok(())
}
