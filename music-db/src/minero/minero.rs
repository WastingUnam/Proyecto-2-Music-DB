use id3::{Tag, TagLike};
use walkdir::{WalkDir, DirEntry};

/// Funcion con una lambda para los diferentes tipos de audio
fn es_audio(entry: &DirEntry) -> bool {
	entry.file_name().to_str()
	.map(|x| x.ends_with(".mp3") || x.ends_with(".flac"))
	.unwrap_or(false)
}

pub fn minero() -> Result<(), Box<dyn std::error::Error>> {
	let directorio = WalkDir::new(".").into_iter().filter_map(|x| x.ok());
	for entry in directorio {

		if !entry.file_type().is_file() { continue; }
		if !es_audio(&entry){ continue; }

		let path = entry.path();
		if let Ok(tag) = Tag::read_from_path(path){

			if let Some(artist) = tag.artist() {
				println!("Artista: {}", artist);
			}
			if let Some(title) = tag.title() {
				println!("Titulo: {}", title);
			}
			if let Some(album) = tag.album() {
				println!("Album: {}", album);
			}
			if let Some(track) = tag.track(){
				println!("Numero de pista: {}", track);
			}
			println!("{}", path.display());
			print!("\n");
		}
	}
	Ok(())
}
