use crate::app_core::models::{Difficulty, Task};
use rusqlite::{Connection, Result};

pub fn insert_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (id, title, difficulty, is_completed, is_pinned, is_urgent, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            task.id,
            task.title,
            task.difficulty.as_str(),
            task.is_completed as i32,
            task.is_pinned as i32,
            task.is_urgent as i32,
            task.created_at,
        ],
    )?;
    Ok(())
}

pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, difficulty, is_completed, is_pinned, is_urgent, created_at, completed_at
         FROM tasks ORDER BY is_pinned DESC, is_urgent DESC, is_completed ASC, created_at DESC"
    )?;

    let tasks = stmt.query_map([], |row| {
        let difficulty_str: String = row.get(2)?;
        let difficulty = match difficulty_str.as_str() {
            "Easy" => Difficulty::Easy,
            "Hard" => Difficulty::Hard,
            "Epic" => Difficulty::Epic,
            _      => Difficulty::Medium,
        };
        Ok(Task {
            id:           row.get(0)?,
            title:        row.get(1)?,
            difficulty,
            is_completed: row.get::<_, i32>(3)? != 0,
            is_pinned:    row.get::<_, i32>(4)? != 0,
            is_urgent:    row.get::<_, i32>(5)? != 0,
            created_at:   row.get(6)?,
            completed_at: row.get(7)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();

    Ok(tasks)
}

pub fn mark_task_completed(conn: &Connection, task_id: &str, completed_at: i64) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET is_completed = 1, completed_at = ?1 WHERE id = ?2",
        rusqlite::params![completed_at, task_id],
    )?;
    Ok(())
}

pub fn get_total_xp(conn: &Connection) -> Result<u32> {
    conn.query_row(
        "SELECT total_xp FROM user_progress WHERE id = 1",
        [],
        |row| row.get::<_, u32>(0),
    )
}

pub fn add_xp(conn: &Connection, xp_to_add: u32) -> Result<()> {
    conn.execute(
        "UPDATE user_progress SET total_xp = total_xp + ?1 WHERE id = 1",
        rusqlite::params![xp_to_add],
    )?;
    Ok(())
}