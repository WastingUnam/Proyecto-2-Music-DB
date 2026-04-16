use chrono::{DateTime, Datelike, Local};
use id3::{Tag, TagLike};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Cancion {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub track: u32,
    pub year: i32,
    pub path: String,
    pub album_path: String,
}

pub fn mp3(path: &Path) -> Cancion {
    let year_from_file = fs::metadata(path)
        .and_then(|x| x.created())
        .ok()
        .map(|x| {
            let fecha: DateTime<Local> = x.into();
            fecha.year()
        })
        .unwrap_or(2026);
    let mut cancion = Cancion {
        title: "Desconocido".to_string(),
        artist: "Desconocido".to_string(),
        album: "Desconocido".to_string(),
        genre: "Desconocido".to_string(),
        track: 1,
        year: year_from_file,
        // Se queja mi compilador si pongo directo el path.
        // Asi que mejor lo pongo despues, no es como que pueda cambiar.
        path: ".".to_string(),
        album_path: ".".to_string(),
    };
    if let Ok(tag) = Tag::read_from_path(path) {
        if let Some(title) = tag.title() {
            cancion.title = title.to_string();
        }
        if let Some(artist) = tag.artist() {
            cancion.artist = artist.to_string();
        }
        if let Some(track) = tag.track() {
            cancion.track = track;
        }
        if let Some(album) = tag.album() {
            cancion.album = album.to_string();
        }
        cancion.album_path = path.parent().unwrap().to_string_lossy().into_owned();
        if let Some(genre) = tag.genre() {
            cancion.genre = genre.to_string();
        }
        // Algunos mp3 guardan su fecha de forma distinta.
        if let Some(year) = tag.year() {
            cancion.year = year;
        } else if let Some(date) = tag.date_recorded() {
            cancion.year = date.year;
        }
        cancion.path = path.to_string_lossy().into_owned();
        println!("{:#?}", cancion);
        print!("\n");
        cancion
    } else {
        cancion
    }
}
