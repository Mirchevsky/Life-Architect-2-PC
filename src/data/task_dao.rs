use crate::app_core::models::{Difficulty, Prerequisite, Task};
use rusqlite::{Connection, Result};

// ---------------------------------------------------------------------------
// Insert
// ---------------------------------------------------------------------------

pub fn insert_task(conn: &Connection, task: &Task) -> Result<()> {
    let prerequisites_json = serde_json::to_string(&task.prerequisites)
        .unwrap_or_else(|_| "[]".to_string());

    conn.execute(
        "INSERT INTO tasks (
            id, title, description, category, difficulty,
            is_completed, is_pinned, is_urgent,
            prerequisites, due_date, goal_id, created_at
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        rusqlite::params![
            task.id,
            task.title,
            task.description,
            task.category,
            task.difficulty.as_str(),
            task.is_completed as i32,
            task.is_pinned as i32,
            task.is_urgent as i32,
            prerequisites_json,
            task.due_date,
            task.goal_id,
            task.created_at,
        ],
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Read
// ---------------------------------------------------------------------------

pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, category, difficulty,
                is_completed, is_pinned, is_urgent,
                prerequisites, due_date, goal_id, created_at, completed_at
         FROM tasks
         ORDER BY is_pinned DESC, is_urgent DESC, is_completed ASC, created_at DESC"
    )?;

    let tasks = stmt.query_map([], |row| {
        let difficulty_str: String = row.get(4)?;
        let difficulty = match difficulty_str.as_str() {
            "Easy" => Difficulty::Easy,
            "Hard" => Difficulty::Hard,
            "Epic" => Difficulty::Epic,
            _      => Difficulty::Medium,
        };

        let prerequisites_json: String = row.get(8).unwrap_or_else(|_| "[]".to_string());
        let prerequisites: Vec<Prerequisite> = serde_json::from_str(&prerequisites_json)
            .unwrap_or_default();

        Ok(Task {
            id:            row.get(0)?,
            title:         row.get(1)?,
            description:   row.get(2).unwrap_or_default(),
            category:      row.get(3).unwrap_or_default(),
            difficulty,
            is_completed:  row.get::<_, i32>(5)? != 0,
            is_pinned:     row.get::<_, i32>(6)? != 0,
            is_urgent:     row.get::<_, i32>(7)? != 0,
            prerequisites,
            due_date:      row.get(9)?,
            goal_id:       row.get(10).unwrap_or_default(),
            created_at:    row.get(11)?,
            completed_at:  row.get(12)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();

    Ok(tasks)
}

// ---------------------------------------------------------------------------
// Update — toggle completion
// ---------------------------------------------------------------------------

pub fn mark_task_completed(conn: &Connection, task_id: &str, completed_at: i64) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET is_completed = 1, completed_at = ?1 WHERE id = ?2",
        rusqlite::params![completed_at, task_id],
    )?;
    Ok(())
}

pub fn mark_task_uncompleted(conn: &Connection, task_id: &str) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET is_completed = 0, completed_at = NULL WHERE id = ?1",
        rusqlite::params![task_id],
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Update — prerequisites
// ---------------------------------------------------------------------------

pub fn update_prerequisites(conn: &Connection, task_id: &str, prerequisites: &[Prerequisite]) -> Result<()> {
    let json = serde_json::to_string(prerequisites).unwrap_or_else(|_| "[]".to_string());
    conn.execute(
        "UPDATE tasks SET prerequisites = ?1 WHERE id = ?2",
        rusqlite::params![json, task_id],
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// XP helpers
// ---------------------------------------------------------------------------

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

pub fn subtract_xp(conn: &Connection, xp_to_subtract: u32) -> Result<()> {
    conn.execute(
        "UPDATE user_progress SET total_xp = MAX(0, total_xp - ?1) WHERE id = 1",
        rusqlite::params![xp_to_subtract],
    )?;
    Ok(())
}