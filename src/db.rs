pub mod ingredients;
pub mod pantry;

use rusqlite::{Connection, Result};

pub fn create_connection_path() -> Result<Connection> {
    let path = "./my_db.db3";
    let connection = Connection::open(path)?;
    connection.execute("PRAGMA foreign_keys = ON;", [])?;
    create_tables(&connection)?;
    Ok(connection)
}

pub fn create_connection_memory() -> Result<Connection> {
    let connection = Connection::open_in_memory()?;
    connection.execute("PRAGMA foreign_keys = ON;", [])?;
    create_tables(&connection)?;
    Ok(connection)
}

fn create_tables(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS ingredients (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        unit TEXT NOT NULL,
        category TEXT NOT NULL)",
        (),
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS pantry (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ingredient_id INTEGER NOT NULL REFERENCES ingredients(id),
        quantity INTEGER NOT NULL,
        expiry_date TEXT NOT NULL)",
        (),
    )?;

    Ok(())
}
