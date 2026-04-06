use std::path::Path;
use id3::{Tag, TagLike};
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use lofty::prelude::Accessor;
use lofty::prelude::ItemKey;

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

pub fn audio_general(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
	let archivo = Probe::open(path)?.read()?;
	if let Some(tag) = archivo.primary_tag() {
		if let Some(artist) = tag.artist().as_deref() {
			println!("Artista: {}", artist);
		}
		if let Some(title) = tag.title().as_deref() {
			println!("Titulo: {}", title);
		}
		if let Some(track) = tag.track() {
			println!("Numero de pista: {}", track);
		}
		if let Some(album) = tag.album().as_deref() {
			println!("Album: {}", album);
		}
		// Lo mismo, no siempre se guarda la fecha de la misma manera.
		if let Some(year) = tag.get_string(ItemKey::Year) {
			println!("Year: {}", year);
		}
		else if let Some(date) = tag.get_string(ItemKey::RecordingDate) {
			println!("Year: {}", date);
		}
		println!("{}", path.display());
		print!("\n");
	}
	Ok(())
}
