use rusqlite::{Connection, Result};
use std::path::PathBuf;

/// Returns the path to the app's data directory: %APPDATA%\Life-Architect-2-PC\
pub fn get_data_dir() -> PathBuf {
    let mut path = dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("Life-Architect-2-PC");
    std::fs::create_dir_all(&path).expect("Failed to create data directory");
    path
}

/// Opens the SQLite database and runs schema initialization.
pub fn open_database() -> Result<Connection> {
    let mut db_path = get_data_dir();
    db_path.push("life_architect.db");
    let conn = Connection::open(&db_path)?;
    initialize_schema(&conn)?;
    Ok(conn)
}

fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS tasks (
            id            TEXT PRIMARY KEY NOT NULL,
            title         TEXT NOT NULL,
            difficulty    TEXT NOT NULL DEFAULT 'Medium',
            is_completed  INTEGER NOT NULL DEFAULT 0,
            is_pinned     INTEGER NOT NULL DEFAULT 0,
            is_urgent     INTEGER NOT NULL DEFAULT 0,
            created_at    INTEGER NOT NULL,
            completed_at  INTEGER
        );

        CREATE TABLE IF NOT EXISTS user_progress (
            id            INTEGER PRIMARY KEY CHECK (id = 1),
            total_xp      INTEGER NOT NULL DEFAULT 0
        );

        INSERT OR IGNORE INTO user_progress (id, total_xp) VALUES (1, 0);
    ")
}