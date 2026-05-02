// DAO, todo lo de sqlite va aqui
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

/// Ruta de la base de datos, va en XDG_DATA_HOME
pub fn db_path() -> PathBuf {
    let mut path = dirs::data_dir().expect("No se pudo obtener XDG_DATA_HOME");
    path.push("music-db");
    fs::create_dir_all(&path).expect("No se pudo crear el directorio de datos");
    path.push("base.db");
    path
}

/// Crea las tablas si no existen
pub fn iniciar_schema(conn: &Connection) {
    conn.execute_batch(SCHEMA)
        .expect("No se pudo ejecutar el schema");
}

/// Si no existe lo crea como unknown
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

/// Igual que el de arriba pero para albums
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

/// Para no meter duplicados
fn rola_existe(conn: &Connection, path: &str) -> bool {
    let mut rola = conn
        .prepare("SELECT 1 FROM rolas WHERE path = ?1")
        .unwrap();
    rola.exists(params![path]).unwrap()
}

/// Mete una cancion a la DB, regresa false si ya estaba
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

/// Saca el id del performer de una rola
pub fn obtener_id_performer_de_rola(id_rola: i64) -> Result<i64, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    conn.query_row(
        "SELECT id_performer FROM rolas WHERE id_rola = ?1",
        params![id_rola],
        |row| row.get(0),
    )
}

/// 0=persona, 1=grupo, 2=unknown
pub fn obtener_tipo_performer(id_performer: i64) -> Result<i32, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    conn.query_row(
        "SELECT id_type FROM performers WHERE id_performer = ?1",
        params![id_performer],
        |row| row.get(0),
    )
}

/// Datos de persona, regresa None si no hay
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

/// Guarda performer como persona, le cambia el tipo y mete los datos
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

/// Datos de grupo, igual que obtener_persona pero para grupos
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

/// Lo mismo que actualizar_performer_persona pero pa grupo
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

/// Nomas el nombre
pub fn obtener_nombre_performer(id_performer: i64) -> Result<String, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    conn.query_row(
        "SELECT name FROM performers WHERE id_performer = ?1",
        params![id_performer],
        |row| row.get(0),
    )
}

/// Todas las personas que hay, para el dialogo de miembros
pub fn obtener_todas_personas() -> Result<Vec<(i64, String)>, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    let mut stmt = conn.prepare(
        "SELECT id_performer, name FROM performers WHERE id_type = 0 ORDER BY name"
    )?;
    let personas = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;
    personas.collect()
}

/// Quienes estan en el grupo
pub fn obtener_miembros_grupo(id_group: i64) -> Result<Vec<i64>, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    let mut stmt = conn.prepare(
        "SELECT id_person FROM in_group WHERE id_group = ?1"
    )?;
    let ids = stmt.query_map(params![id_group], |row| row.get(0))?;
    ids.collect()
}

/// Borra los que habia y mete los nuevos, con transaccion pa que no se rompa
pub fn actualizar_miembros_grupo(id_group: i64, ids_personas: &[i64]) -> Result<(), rusqlite::Error> {
    let mut conn = Connection::open(db_path())?;
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM in_group WHERE id_group = ?1", params![id_group])?;
    for &id_person in ids_personas {
        tx.execute(
            "INSERT INTO in_group (id_person, id_group) VALUES (?1, ?2)",
            params![id_person, id_group],
        )?;
    }
    tx.commit()?;
    Ok(())
}

/// Crea persona nueva, mete en performers y en persons
pub fn crear_persona(nombre: &str) -> Result<i64, rusqlite::Error> {
    let conn = Connection::open(db_path())?;
    conn.execute(
        "INSERT INTO performers (id_type, name) VALUES (0, ?1)",
        params![nombre],
    )?;
    let id = conn.last_insert_rowid();
    conn.execute(
        "INSERT INTO persons (id_person, stage_name, real_name, birth_date, death_date) \
         VALUES (?1, ?2, '', '', '')",
        params![id, nombre],
    )?;
    Ok(id)
}

/// Busca rolas usando una query con lógica proposicional
pub fn buscar_rolas(query: &str) -> Result<Vec<RolaView>, String> {
    let expr = crate::compilador::parsear(query)?;
    let query_sql = crate::compilador::query_builder::expr_a_sql(&expr);

    let db = db_path();
    if !db.exists() {
        return Ok(Vec::new());
    }
    let conn = Connection::open(&db).map_err(|e| e.to_string())?;

    let sql = format!(
        "SELECT r.id_rola, r.title, COALESCE(a.name, 'Desconocido'), \
         COALESCE(p.name, 'Desconocido'), r.track, r.year, COALESCE(r.genre, 'Desconocido'), r.path \
         FROM rolas r \
         LEFT JOIN albums a ON r.id_album = a.id_album \
         LEFT JOIN performers p ON r.id_performer = p.id_performer \
         WHERE {} \
         ORDER BY a.name, r.track",
        query_sql.where_clause
    );

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = query_sql
        .params
        .iter()
        .map(|s| s as &dyn rusqlite::types::ToSql)
        .collect();

    let rolas = stmt
        .query_map(param_refs.as_slice(), |row| {
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
        })
        .map_err(|e| e.to_string())?;

    rolas
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

/// Trae todas las rolas con join a album y performer pa la tabla
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
