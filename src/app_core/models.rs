use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

// ---------------------------------------------------------------------------
// Difficulty
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Epic,
}

impl Difficulty {
    pub fn as_str(&self) -> &str {
        match self {
            Difficulty::Easy   => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard   => "Hard",
            Difficulty::Epic   => "Epic",
        }
    }

    /// XP reward for completing a task of this difficulty.
    /// Mirrors the Android app's XP values.
    pub fn xp_reward(&self) -> u32 {
        match self {
            Difficulty::Easy   => 10,
            Difficulty::Medium => 25,
            Difficulty::Hard   => 50,
            Difficulty::Epic   => 100,
        }
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ---------------------------------------------------------------------------
// Prerequisite — an embedded sub-task (not a separate DB table)
// Mirrors the Android Prerequisite data class inside TaskEntity.kt
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prerequisite {
    pub id: String,
    pub label: String,
    pub completed: bool,
    pub completed_at: Option<i64>,
}

#[allow(dead_code)]
impl Prerequisite {
    pub fn new(label: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            label,
            completed: false,
            completed_at: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Task
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub difficulty: Difficulty,
    pub is_completed: bool,
    pub is_pinned: bool,
    pub is_urgent: bool,
    pub prerequisites: Vec<Prerequisite>,
    pub due_date: Option<i64>,
    pub goal_id: String,
    pub created_at: i64,
    pub completed_at: Option<i64>,
}

impl Task {
    pub fn new(title: String, difficulty: Difficulty) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description: String::new(),
            category: String::new(),
            difficulty,
            is_completed: false,
            is_pinned: false,
            is_urgent: false,
            prerequisites: Vec::new(),
            due_date: None,
            goal_id: String::new(),
            created_at: Utc::now().timestamp_millis(),
            completed_at: None,
        }
    }

    /// Returns true if all prerequisites are completed (or there are none).
    /// A task cannot be toggled to completed unless this returns true.
    pub fn prerequisites_met(&self) -> bool {
        self.prerequisites.iter().all(|p| p.completed)
    }
}

// ---------------------------------------------------------------------------
// Goal — mirrors Android GoalEntity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Goal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub color_hex: String,
    pub created_at: i64,
}

impl Goal {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description: String::new(),
            color_hex: String::from("#FFFFFF"),
            created_at: Utc::now().timestamp_millis(),
        }
    }
}