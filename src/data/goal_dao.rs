use crate::app_core::models::Goal;
use rusqlite::{Connection, Result};

// ---------------------------------------------------------------------------
// Insert
// ---------------------------------------------------------------------------

pub fn insert_goal(conn: &Connection, goal: &Goal) -> Result<()> {
    conn.execute(
        "INSERT INTO goals (id, title, description, color_hex, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            goal.id,
            goal.title,
            goal.description,
            goal.color_hex,
            goal.created_at,
        ],
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Read
// ---------------------------------------------------------------------------

pub fn get_all_goals(conn: &Connection) -> Result<Vec<Goal>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, color_hex, created_at
         FROM goals
         ORDER BY created_at DESC"
    )?;

    let goals = stmt.query_map([], |row| {
        Ok(Goal {
            id:          row.get(0)?,
            title:       row.get(1)?,
            description: row.get(2).unwrap_or_default(),
            color_hex:   row.get(3).unwrap_or_else(|_| "#FFFFFF".to_string()),
            created_at:  row.get(4)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();

    Ok(goals)
}

// ---------------------------------------------------------------------------
// Delete
// ---------------------------------------------------------------------------

pub fn delete_goal(conn: &Connection, goal_id: &str) -> Result<()> {
    // Unlink tasks that belonged to this goal (set goal_id to empty string)
    conn.execute(
        "UPDATE tasks SET goal_id = '' WHERE goal_id = ?1",
        rusqlite::params![goal_id],
    )?;
    conn.execute(
        "DELETE FROM goals WHERE id = ?1",
        rusqlite::params![goal_id],
    )?;
    Ok(())
}