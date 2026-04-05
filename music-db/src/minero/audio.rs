use std::path::Path;
use id3::{Tag, TagLike};
use lofty::read_from_path;
use lofty::file::TaggedFileExt;
use lofty::prelude::Accessor;

pub fn mp3(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
	Ok(())
}

pub fn audio_general(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
	if let Ok(archivo) = read_from_path(path) {
		if let Some(tag) = archivo.primary_tag(){
			if let Some(artist) = tag.artist().as_deref() {
				println!("Artista: {}", artist);
			}
			if let Some(title) = tag.title().as_deref() {
				println!("Titulo: {}", title);
			}
			if let Some(album) = tag.album().as_deref() {
				println!("Album: {}", album);
			}
			if let Some(track) = tag.track() {
				println!("Numero de pista: {}", track);
			}
			println!("{}", path.display());
			println!("\n");
		}
	}
	Ok(())
}
