// Toda la logica de acceso a la base de datos SQLite.
use crate::minero::audio::Cancion;
use rusqlite::{Connection, params};
use std::fs;
use std::path::PathBuf;

const SCHEMA: &str = include_str!("../../SQL/schema.sql");

#[derive(Debug, Clone)]
pub struct RolaView {
    pub id_rola: i64,
    pub title: String,
    pub album: String,
    pub performer: String,
    pub track: u32,
    pub year: i32,
    pub genre: String,
    pub path: String,
}

/// Ruta donde se guarda la base de datos.
pub fn db_path() -> PathBuf {
    let mut path = dirs::data_dir().expect("No se pudo obtener XDG_DATA_HOME");
    path.push("music-db");
    fs::create_dir_all(&path).expect("No se pudo crear el directorio de datos");
    path.push("base.db");
    path
}

/// Crear las tablas si no existen todavia.
pub fn iniciar_schema(conn: &Connection) {
    conn.execute_batch(SCHEMA)
        .expect("No se pudo ejecutar el schema");
}

/// Busca un performer por nombre, si no existe lo crea como Unknown (tipo 2).
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

/// Busca un album por nombre, si no existe lo crea.
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

/// Checar si ya tenemos esta cancion en la DB para no duplicar.
fn rola_existe(conn: &Connection, path: &str) -> bool {
    let mut rola = conn
        .prepare("SELECT 1 FROM rolas WHERE path = ?1")
        .unwrap();
    rola.exists(params![path]).unwrap()
}

/// Insertar una cancion en la DB, regresa false si ya existe esa cancion.
pub fn insertar_cancion(conn: &Connection, cancion: &Cancion) -> Result<bool, rusqlite::Error> {
    if rola_existe(conn, &cancion.path) {
        return Ok(false);
    }

    let id_performer = buscar_o_crear_performer(conn, &cancion.artist);
    let id_album = buscar_o_crear_album(conn, &cancion.album, cancion.year, &cancion.album_path);

    conn.execute(
        "INSERT INTO rolas (id_performer, id_album, path, title, track, year, genre)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id_performer, id_album, cancion.path, cancion.title, cancion.track, cancion.year, cancion.genre],
    )?;

    Ok(true)
}

/// Sacar el id del performer a partir de una rola.
pub fn obtener_id_performer_de_rola(id_rola: i64) -> Result<i64, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    conn.query_row(
        "SELECT id_performer FROM rolas WHERE id_rola = ?1",
        params![id_rola],
        |row| row.get(0),
    )
}

/// Sacar el tipo de performer (0=persona, 1=grupo, 2=unknown).
pub fn obtener_tipo_performer(id_performer: i64) -> Result<i32, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    conn.query_row(
        "SELECT id_type FROM performers WHERE id_performer = ?1",
        params![id_performer],
        |row| row.get(0),
    )
}

/// Sacar los datos de una persona (stage name, nombre real, fechas).
pub fn obtener_persona(id_performer: i64) -> Result<Option<(String, String, String, String)>, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    let resultado = conn.query_row(
        "SELECT stage_name, real_name, birth_date, death_date FROM persons WHERE id_person = ?1",
        params![id_performer],
        |row| {
            Ok((
                row.get::<_, String>(0).unwrap_or_default(),
                row.get::<_, String>(1).unwrap_or_default(),
                row.get::<_, String>(2).unwrap_or_default(),
                row.get::<_, String>(3).unwrap_or_default(),
            ))
        },
    );
    match resultado {
        Ok(datos) => Ok(Some(datos)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Guardar los datos de un performer como persona.
pub fn actualizar_performer_persona(
    id_performer: i64,
    stage_name: &str,
    real_name: &str,
    birth_date: &str,
    death_date: &str,
) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "UPDATE performers SET id_type = 0 WHERE id_performer = ?1",
        params![id_performer],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO persons (id_person, stage_name, real_name, birth_date, death_date) \
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id_performer, stage_name, real_name, birth_date, death_date],
    )?;
    Ok(())
}

/// Sacar los datos de un grupo (nombre, fecha inicio, fecha fin).
pub fn obtener_grupo(id_performer: i64) -> Result<Option<(String, String, String)>, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    let resultado = conn.query_row(
        "SELECT name, start_date, end_date FROM groups WHERE id_group = ?1",
        params![id_performer],
        |row| {
            Ok((
                row.get::<_, String>(0).unwrap_or_default(),
                row.get::<_, String>(1).unwrap_or_default(),
                row.get::<_, String>(2).unwrap_or_default(),
            ))
        },
    );
    match resultado {
        Ok(datos) => Ok(Some(datos)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Guardar los datos de un performer como grupo.
pub fn actualizar_performer_grupo(
    id_performer: i64,
    start_date: &str,
    end_date: &str,
) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    let nombre = conn.query_row(
        "SELECT name FROM performers WHERE id_performer = ?1",
        params![id_performer],
        |row| row.get::<_, String>(0),
    )?;
    conn.execute(
        "UPDATE performers SET id_type = 1 WHERE id_performer = ?1",
        params![id_performer],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO groups (id_group, name, start_date, end_date) \
         VALUES (?1, ?2, ?3, ?4)",
        params![id_performer, nombre, start_date, end_date],
    )?;
    Ok(())
}

/// Sacar el nombre de un performer por su id.
pub fn obtener_nombre_performer(id_performer: i64) -> Result<String, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    conn.query_row(
        "SELECT name FROM performers WHERE id_performer = ?1",
        params![id_performer],
        |row| row.get(0),
    )
}

/// Traer todas las rolas de la DB con sus joins a album y performer.
pub fn obtener_rolas() -> Result<Vec<RolaView>, rusqlite::Error> {
    let db = db_path();
    if !db.exists() {
        return Ok(Vec::new());
    }
    let conn = Connection::open(&db)?;

    let mut stmt = conn.prepare(
        "SELECT r.id_rola, r.title, COALESCE(a.name, 'Desconocido'), \
         COALESCE(p.name, 'Desconocido'), r.track, r.year, COALESCE(r.genre, 'Desconocido'), r.path \
         FROM rolas r \
         LEFT JOIN albums a ON r.id_album = a.id_album \
         LEFT JOIN performers p ON r.id_performer = p.id_performer \
         ORDER BY a.name, r.track"
    )?;

    let rolas = stmt.query_map([], |row| {
        Ok(RolaView {
            id_rola: row.get(0)?,
            title: row.get(1)?,
            album: row.get(2)?,
            performer: row.get(3)?,
            track: row.get(4)?,
            year: row.get(5)?,
            genre: row.get(6)?,
            path: row.get(7)?,
        })
    })?;

    rolas.collect()
}
