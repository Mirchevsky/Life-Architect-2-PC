use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Epic,
}

impl Difficulty {
    pub fn xp_value(&self) -> u32 {
        match self {
            Difficulty::Easy   => 10,
            Difficulty::Medium => 25,
            Difficulty::Hard   => 50,
            Difficulty::Epic   => 100,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::Easy   => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard   => "Hard",
            Difficulty::Epic   => "Epic",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub is_completed: bool,
    pub is_pinned: bool,
    pub is_urgent: bool,
    pub created_at: i64,
    pub completed_at: Option<i64>,
}

impl Task {
    pub fn new(title: String, difficulty: Difficulty) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            difficulty,
            is_completed: false,
            is_pinned: false,
            is_urgent: false,
            created_at: Utc::now().timestamp_millis(),
            completed_at: None,
        }
    }
}