use crate::app_core::models::{Difficulty, Goal, Task};
use crate::app_core::engine;
use crate::data::daily_quote_engine::DailyQuote;
use crate::data::global_events_engine::GlobalEvent;
use iced::widget::{button, column, container, progress_bar, row, text};
use iced::{Element, Length};
use rusqlite::Connection;

// ---------------------------------------------------------------------------
// Screen
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Dashboard,
    Tasks,
    Goals,
    Analytics,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Dashboard
    }
}

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

pub struct LifeArchitect {
    pub current_screen: Screen,
    pub tasks: Vec<Task>,
    pub goals: Vec<Goal>,
    pub total_xp: u32,
    pub conn: Connection,

    // Task creation form fields
    pub new_task_input: String,
    pub new_task_description: String,
    pub new_task_category: String,
    pub new_task_due_date: String,
    pub selected_difficulty: Difficulty,

    // Goal creation form field
    pub new_goal_input: String,

    // v0.4 — daily content (loaded once at startup, stable for the whole day)
    pub daily_quote: DailyQuote,
    pub today_event: GlobalEvent,
    pub tomorrow_event_title: String,
}

impl LifeArchitect {
    pub fn new(
        conn: Connection,
        tasks: Vec<Task>,
        goals: Vec<Goal>,
        total_xp: u32,
        daily_quote: DailyQuote,
        today_event: GlobalEvent,
        tomorrow_event_title: String,
    ) -> Self {
        Self {
            current_screen: Screen::Dashboard,
            tasks,
            goals,
            total_xp,
            conn,
            new_task_input: String::new(),
            new_task_description: String::new(),
            new_task_category: String::new(),
            new_task_due_date: String::new(),
            selected_difficulty: Difficulty::Medium,
            new_goal_input: String::new(),
            daily_quote,
            today_event,
            tomorrow_event_title,
        }
    }
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Screen),

    // Task form
    NewTaskInputChanged(String),
    NewTaskDescriptionChanged(String),
    NewTaskCategoryChanged(String),
    NewTaskDueDateChanged(String),
    DifficultySelected(Difficulty),
    AddTask,
    ToggleTask(String),

    // Goal form
    NewGoalInputChanged(String),
    AddGoal,
    DeleteGoal(String),
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

impl LifeArchitect {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NavigateTo(screen) => {
                self.current_screen = screen;
            }

            // --- Task form inputs ---
            Message::NewTaskInputChanged(value) => {
                self.new_task_input = value;
            }
            Message::NewTaskDescriptionChanged(value) => {
                self.new_task_description = value;
            }
            Message::NewTaskCategoryChanged(value) => {
                self.new_task_category = value;
            }
            Message::NewTaskDueDateChanged(value) => {
                self.new_task_due_date = value;
            }
            Message::DifficultySelected(difficulty) => {
                self.selected_difficulty = difficulty;
            }

            // --- Add task ---
            Message::AddTask => {
                let title = self.new_task_input.trim().to_string();
                if title.is_empty() {
                    return;
                }

                let mut task = Task::new(title, self.selected_difficulty.clone());
                task.description = self.new_task_description.trim().to_string();
                task.category    = self.new_task_category.trim().to_string();

                if !self.new_task_due_date.trim().is_empty() {
                    if let Ok(date) = chrono::NaiveDate::parse_from_str(
                        self.new_task_due_date.trim(), "%Y-%m-%d",
                    ) {
                        let dt = date.and_hms_opt(0, 0, 0)
                            .map(|ndt| ndt.and_utc().timestamp_millis());
                        task.due_date = dt;
                    }
                }

                let _ = crate::data::task_dao::insert_task(&self.conn, &task);
                self.tasks.push(task);

                self.new_task_input.clear();
                self.new_task_description.clear();
                self.new_task_category.clear();
                self.new_task_due_date.clear();
            }

            // --- Toggle task completion ---
            Message::ToggleTask(task_id) => {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
                    if task.is_completed {
                        let xp_reward = task.difficulty.xp_reward();
                        task.is_completed = false;
                        task.completed_at = None;
                        let _ = crate::data::task_dao::mark_task_uncompleted(
                            &self.conn, &task.id,
                        );
                        let _ = crate::data::task_dao::subtract_xp(&self.conn, xp_reward);
                        self.total_xp = self.total_xp.saturating_sub(xp_reward);
                    } else {
                        if !task.prerequisites_met() {
                            return;
                        }
                        let xp_reward = task.difficulty.xp_reward();
                        let completed_at = chrono::Utc::now().timestamp_millis();
                        task.is_completed = true;
                        task.completed_at = Some(completed_at);
                        let _ = crate::data::task_dao::mark_task_completed(
                            &self.conn, &task.id, completed_at,
                        );
                        let _ = crate::data::task_dao::add_xp(&self.conn, xp_reward);
                        self.total_xp += xp_reward;
                    }
                }
            }

            // --- Goal form ---
            Message::NewGoalInputChanged(value) => {
                self.new_goal_input = value;
            }
            Message::AddGoal => {
                let title = self.new_goal_input.trim().to_string();
                if title.is_empty() {
                    return;
                }
                let goal = Goal::new(title);
                let _ = crate::data::goal_dao::insert_goal(&self.conn, &goal);
                self.goals.push(goal);
                self.new_goal_input.clear();
            }
            Message::DeleteGoal(goal_id) => {
                let _ = crate::data::goal_dao::delete_goal(&self.conn, &goal_id);
                self.goals.retain(|g| g.id != goal_id);
                for task in self.tasks.iter_mut() {
                    if task.goal_id == goal_id {
                        task.goal_id.clear();
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

impl LifeArchitect {
    pub fn view(&self) -> Element<'_, Message> {
        let level       = engine::calculate_level(self.total_xp);
        let xp_in_level = engine::xp_within_current_level(self.total_xp) as f32;
        let xp_needed   = engine::xp_for_next_level(level) as f32;
        let xp_progress = if xp_needed > 0.0 { xp_in_level / xp_needed } else { 0.0 };

        // --- Sidebar ---
        let sidebar = column![
            text("Life Architect 2").size(20),
            text("").size(8),
            text(format!("Level {}", level)).size(16),
            text(format!("{} / {} XP", xp_in_level as u32, xp_needed as u32)).size(12),
            progress_bar(0.0..=1.0, xp_progress),
            text("").size(16),
            button("Dashboard")
                .on_press(Message::NavigateTo(Screen::Dashboard))
                .width(Length::Fill),
            button("  Tasks  ")
                .on_press(Message::NavigateTo(Screen::Tasks))
                .width(Length::Fill),
            button("  Goals  ")
                .on_press(Message::NavigateTo(Screen::Goals))
                .width(Length::Fill),
            button("Analytics")
                .on_press(Message::NavigateTo(Screen::Analytics))
                .width(Length::Fill),
        ]
        .spacing(6)
        .padding(16)
        .width(Length::Fixed(200.0));

        // --- Tasks-done-today count (for the dashboard stats row) ---
        let today = chrono::Local::now().date_naive();
        let tasks_done_today = self.tasks.iter().filter(|t| {
            if let Some(ms) = t.completed_at {
                chrono::DateTime::from_timestamp_millis(ms)
                    .map(|dt: chrono::DateTime<chrono::Utc>| {
                        dt.with_timezone(&chrono::Local).date_naive()
                    })
                    .unwrap_or_default()
                    == today
            } else {
                false
            }
        }).count();

        // --- Main content ---
        let main_content: Element<Message> = match self.current_screen {
            Screen::Dashboard => crate::ui::screens::dashboard::view(
                &self.daily_quote,
                &self.today_event,
                &self.tomorrow_event_title,
                level,
                self.total_xp,
                tasks_done_today,
            ),
            Screen::Tasks => crate::ui::screens::task_list::view(
                &self.tasks,
                &self.new_task_input,
                &self.new_task_description,
                &self.new_task_category,
                &self.new_task_due_date,
                &self.selected_difficulty,
            ),
            Screen::Goals => crate::ui::screens::goals::view(
                &self.goals,
                &self.tasks,
                &self.new_goal_input,
            ),
            Screen::Analytics => container(
                column![
                    text("Analytics").size(24),
                    text("").size(8),
                    text(format!("Total XP earned: {}", self.total_xp)).size(16),
                    text(format!("Current level: {}", level)).size(16),
                    text(format!(
                        "Tasks completed: {}",
                        self.tasks.iter().filter(|t| t.is_completed).count()
                    )).size(16),
                    text(format!(
                        "Tasks pending: {}",
                        self.tasks.iter().filter(|t| !t.is_completed).count()
                    )).size(16),
                    text(format!("Goals created: {}", self.goals.len())).size(16),
                ]
                .spacing(8)
                .padding(20),
            ).into(),
        };

        row![sidebar, main_content].into()
    }
}