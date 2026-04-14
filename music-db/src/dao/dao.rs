use crate::minero::audio::Cancion;
use rusqlite::{Connection, Result};
use std::collections::HashMap;

pub fn conecta_db(canciones: &Vec<Cancion>) -> Result<(), Box<dyn std::error::Error>> {
    let conn: Connection = Connection::open("SQL/base.db")?;

    let mut albumes = HashMap::new();

    for cancion in canciones {
        if !albumes.contains_key(&cancion.album) {
            albumes.insert(
                cancion.album.clone(),
                (cancion.year.clone(), cancion.album_path.clone()),
            );
        } else {
            continue;
        }
    }

    for (album, (year, path)) in &albumes {
        conn.execute(
            "INSERT INTO albums (path, name, year)
        VALUES (?1, ?2, ?3)",
            (path, album, year),
        )?;
    }

    Ok(())
}
