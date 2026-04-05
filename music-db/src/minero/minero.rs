use walkdir::{WalkDir, DirEntry};
use super::audio;

fn es_mp3(entry: &DirEntry) -> bool {
	entry.file_name().to_str()
	.map(|x| x.ends_with(".mp3"))
	.unwrap_or(false)
}

/// No se me ocurren otros tipos de audio, y estos son los que uso yo.
fn es_audio(entry: &DirEntry) ->bool {
	entry.file_name().to_str()
	.map(|x| x.ends_with(".flac") || x.ends_with(".wav"))
	.unwrap_or(false)
}

pub fn mina(ruta: &str) -> Result<(), Box<dyn std::error::Error>> {
	let directorio = WalkDir::new(ruta).into_iter().filter_map(|x| x.ok());

	for entry in directorio {
		if !entry.file_type().is_file() { continue; }

		// Pasamos el archivo específico, no la carpeta
		if es_mp3(&entry) {
			let _ = audio::mp3(entry.path());
		} else if es_audio(&entry) {
			let _ = audio::audio_general(entry.path());
		}
	}
	Ok(())
}
