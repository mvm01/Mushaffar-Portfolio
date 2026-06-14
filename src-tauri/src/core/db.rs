use rusqlite::{Connection, Result};

pub fn init_database() -> Result<Connection> {
    let connection = Connection::open("local_data_manager.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS user_profiles (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    Ok(connection)
}
