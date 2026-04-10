use rusqlite::Connection;

pub fn conecta_db() -> Result<(), Box<dyn std::error::Error>> {
    let conn: Connection = Connection::open("SQL/base.db")?;
    Ok(())
}
