use crate::minero::audio::Cancion;
use rusqlite::{Connection, params};
use std::path::Path;

const DB_PATH: &str = "SQL/base.db";
const SCHEMA_PATH: &str = "SQL/schema.sql";

fn iniciar_schema(conn: &Connection) {
    let schema = std::fs::read_to_string(SCHEMA_PATH)
        .expect("No se pudo leer el schema");
    conn.execute_batch(&schema)
        .expect("No se pudo ejecutar el schema");
}

fn buscar_o_crear_performer(conn: &Connection, name: &str) -> i64 {
    let mut performer = conn
        .prepare("SELECT id_performer FROM performers WHERE name = ?1")
        .unwrap();
    let resultado: Option<i64> = performer
        .query_row(params![name], |row| row.get(0))
        .ok();

    match resultado {
        Some(id) => id,
        None => {
            conn.execute(
                "INSERT INTO performers (id_type, name) VALUES (?1, ?2)",
                params![2, name],
            )
            .unwrap();
            conn.last_insert_rowid()
        }
    }
}

fn buscar_o_crear_album(conn: &Connection, name: &str, year: i32, path: &str) -> i64 {
    let mut album = conn
        .prepare("SELECT id_album FROM albums WHERE name = ?1 AND year = ?2")
        .unwrap();
    let resultado: Option<i64> = album
        .query_row(params![name, year], |row| row.get(0))
        .ok();

    match resultado {
        Some(id) => id,
        None => {
            conn.execute("INSERT INTO albums (path, name, year) VALUES (?1, ?2, ?3)",params![path, name, year],)
            .unwrap();
            conn.last_insert_rowid()
        }
    }
}

fn rola_existe(conn: &Connection, path: &str) -> bool {
    let mut rola = conn
        .prepare("SELECT 1 FROM rolas WHERE path = ?1")
        .unwrap();
    rola.exists(params![path]).unwrap()
}

pub fn conecta_db(canciones: &Vec<Cancion>) -> Result<(), Box<dyn std::error::Error>> {
    let db_existe = Path::new(DB_PATH).exists();
    let conn = Connection::open(DB_PATH)?;

    if !db_existe {
        iniciar_schema(&conn);
    }

    let mut nuevas = 0;

    for cancion in canciones {
        if rola_existe(&conn, &cancion.path) {
            continue;
        }

        let id_performer = buscar_o_crear_performer(&conn, &cancion.artist);
        let id_album = buscar_o_crear_album(&conn, &cancion.album, cancion.year, &cancion.album_path,);

        conn.execute("INSERT INTO rolas (id_performer, id_album, path, title, track, year, genre)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id_performer, id_album, cancion.path, cancion.title, cancion.track, cancion.year, cancion.genre,],)?;

        nuevas += 1;
    }

    println!("Se insertaron {} canciones nuevas.", nuevas);
    Ok(())
}
