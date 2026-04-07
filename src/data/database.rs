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

/// Opens the SQLite database and runs schema initialization and migration.
pub fn open_database() -> Result<Connection> {
    let mut db_path = get_data_dir();
    db_path.push("life_architect.db");
    let conn = Connection::open(&db_path)?;
    initialize_schema(&conn)?;
    run_migrations(&conn)?;
    Ok(conn)
}

fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS tasks (
            id              TEXT PRIMARY KEY NOT NULL,
            title           TEXT NOT NULL,
            difficulty      TEXT NOT NULL DEFAULT 'Medium',
            is_completed    INTEGER NOT NULL DEFAULT 0,
            is_pinned       INTEGER NOT NULL DEFAULT 0,
            is_urgent       INTEGER NOT NULL DEFAULT 0,
            created_at      INTEGER NOT NULL,
            completed_at    INTEGER,
            description     TEXT NOT NULL DEFAULT '',
            category        TEXT NOT NULL DEFAULT '',
            due_date        INTEGER,
            prerequisites   TEXT NOT NULL DEFAULT '[]',
            goal_id         TEXT NOT NULL DEFAULT ''
        );

        CREATE TABLE IF NOT EXISTS goals (
            id              TEXT PRIMARY KEY NOT NULL,
            title           TEXT NOT NULL,
            description     TEXT NOT NULL DEFAULT '',
            color_hex       TEXT NOT NULL DEFAULT '#FFFFFF',
            created_at      INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS user_progress (
            id              INTEGER PRIMARY KEY CHECK (id = 1),
            total_xp        INTEGER NOT NULL DEFAULT 0
        );

        INSERT OR IGNORE INTO user_progress (id, total_xp) VALUES (1, 0);
    ")
}

/// Adds new columns to existing databases that were created before v0.3.
/// SQLite does not support adding multiple columns in one ALTER TABLE statement,
/// so each column is added individually and errors are silently ignored if the
/// column already exists (which is the correct migration pattern for SQLite).
fn run_migrations(conn: &Connection) -> Result<()> {
    let migrations = [
        "ALTER TABLE tasks ADD COLUMN description   TEXT NOT NULL DEFAULT ''",
        "ALTER TABLE tasks ADD COLUMN category      TEXT NOT NULL DEFAULT ''",
        "ALTER TABLE tasks ADD COLUMN due_date      INTEGER",
        "ALTER TABLE tasks ADD COLUMN prerequisites TEXT NOT NULL DEFAULT '[]'",
        "ALTER TABLE tasks ADD COLUMN goal_id       TEXT NOT NULL DEFAULT ''",
    ];

    for sql in &migrations {
        // Ignore "duplicate column name" errors — this is expected on re-runs
        let _ = conn.execute_batch(sql);
    }

    // Create goals table if it doesn't exist (for databases upgraded from v0.1)
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS goals (
            id          TEXT PRIMARY KEY NOT NULL,
            title       TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            color_hex   TEXT NOT NULL DEFAULT '#FFFFFF',
            created_at  INTEGER NOT NULL
        );
    ")?;

    Ok(())
}