use crate::app_core::models::{Difficulty, Task};
use crate::app_core::engine;
use iced::widget::{button, column, container, progress_bar, row, text};
use iced::{Element, Length};
use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Tasks,
    Analytics,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Tasks
    }
}

pub struct LifeArchitect {
    pub current_screen: Screen,
    pub tasks: Vec<Task>,
    pub total_xp: u32,
    pub new_task_input: String,
    pub conn: Connection,
}

impl LifeArchitect {
    pub fn new(conn: Connection, tasks: Vec<Task>, total_xp: u32) -> Self {
        Self {
            current_screen: Screen::Tasks,
            tasks,
            total_xp,
            new_task_input: String::new(),
            conn,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Screen),
    NewTaskInputChanged(String),
    AddTask,
    ToggleTask(String),
}

impl LifeArchitect {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NavigateTo(screen) => {
                self.current_screen = screen;
            }
            Message::NewTaskInputChanged(value) => {
                self.new_task_input = value;
            }
            Message::AddTask => {
                if !self.new_task_input.trim().is_empty() {
                    let new_task = Task::new(
                        self.new_task_input.trim().to_string(),
                        Difficulty::Medium,
                    );
                    let _ = crate::data::task_dao::insert_task(&self.conn, &new_task);
                    self.tasks.push(new_task);
                    self.new_task_input.clear();
                }
            }
            Message::ToggleTask(task_id) => {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
                    if !task.is_completed {
                        task.is_completed = true;
                        let xp_reward = task.difficulty.xp_value();
                        let completed_at = chrono::Utc::now().timestamp_millis();
                        task.completed_at = Some(completed_at);
                        let _ = crate::data::task_dao::mark_task_completed(
                            &self.conn, &task.id, completed_at,
                        );
                        let _ = crate::data::task_dao::add_xp(&self.conn, xp_reward);
                        self.total_xp += xp_reward;
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // --- Sidebar ---
        let level = engine::calculate_level(self.total_xp);
        let xp_in_level = engine::xp_within_current_level(self.total_xp) as f32;
        let xp_needed = engine::xp_for_next_level(level) as f32;
        let xp_progress = if xp_needed > 0.0 { xp_in_level / xp_needed } else { 0.0 };

        let sidebar = column![
            text("Life Architect 2").size(20),
            text("").size(8), // spacer
            text(format!("Level {}", level)).size(16),
            text(format!("{} / {} XP", xp_in_level as u32, xp_needed as u32))
                .size(12),
            progress_bar(0.0..=1.0, xp_progress),
            text("").size(16), // spacer
            button("  Tasks  ")
                .on_press(Message::NavigateTo(Screen::Tasks))
                .width(Length::Fill),
            button("Analytics")
                .on_press(Message::NavigateTo(Screen::Analytics))
                .width(Length::Fill),
        ]
        .spacing(6)
        .padding(16)
        .width(Length::Fixed(200.0));

        // --- Main Content ---
        let main_content: Element<Message> = match self.current_screen {
            Screen::Tasks => crate::ui::screens::task_list::view(
                &self.tasks,
                &self.new_task_input,
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
                ]
                .spacing(8)
                .padding(20),
            ).into(),
        };

        row![sidebar, main_content].into()
    }
}