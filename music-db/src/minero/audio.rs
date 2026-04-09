use std::path::Path;
use id3::{Tag, TagLike};

pub fn mp3(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
	if let Ok(tag) = Tag::read_from_path(path){
		if let Some(artist) = tag.artist() {
			println!("Artista: {}", artist);
		}
		if let Some(title) = tag.title() {
			println!("Titulo: {}", title);
		}
		if let Some(track) = tag.track(){
			println!("Numero de pista: {}", track);
		}
		if let Some(album) = tag.album() {
			println!("Album: {}", album);
		}
		// Algunos mp3 guardan su fecha de forma distinta.
		if let Some(year) = tag.year() {
			println!("Year: {}", year);
		}
		else if let Some(date) = tag.date_recorded() {
			println!("Year: {}", date.year);
		}
		println!("{}", path.display());
		print!("\n");
	}
	Ok(())
}

