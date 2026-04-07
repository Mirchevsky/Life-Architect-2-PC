use crate::app::Message;
use crate::app_core::models::{Difficulty, Task};
use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable,
    text, text_input, Space,
};
use iced::{Element, Length};

pub fn view<'a>(
    tasks: &'a [Task],
    title_input: &'a str,
    description_input: &'a str,
    category_input: &'a str,
    due_date_input: &'a str,
    selected_difficulty: &'a Difficulty,
) -> Element<'a, Message> {

    let difficulty_options: &[Difficulty] = &[
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
        Difficulty::Epic,
    ];

    // --- Creation form ---
    let form = column![
        // Row 1: title + difficulty picker + add button
        row![
            text_input("Task title...", title_input)
                .on_input(Message::NewTaskInputChanged)
                .on_submit(Message::AddTask)
                .padding(10)
                .width(Length::Fill),
            pick_list(
                difficulty_options,
                Some(selected_difficulty.clone()),
                Message::DifficultySelected,
            )
            .width(Length::Fixed(120.0)),
            button("Add Task")
                .on_press(Message::AddTask)
                .padding(10),
        ]
        .spacing(8),

        // Row 2: description
        text_input("Description (optional)...", description_input)
            .on_input(Message::NewTaskDescriptionChanged)
            .padding(8)
            .width(Length::Fill),

        // Row 3: category + due date
        row![
            text_input("Category (optional)...", category_input)
                .on_input(Message::NewTaskCategoryChanged)
                .padding(8)
                .width(Length::Fill),
            text_input("Due date (YYYY-MM-DD)...", due_date_input)
                .on_input(Message::NewTaskDueDateChanged)
                .padding(8)
                .width(Length::Fixed(260.0)),
        ]
        .spacing(8),
    ]
    .spacing(6)
    .padding(16);

    // --- Sort: pinned → urgent → pending → completed ---
    let mut sorted: Vec<&Task> = tasks.iter().collect();
    sorted.sort_by(|a, b| {
        let score = |t: &&Task| -> u8 {
            if t.is_pinned           { 0 }
            else if t.is_urgent      { 1 }
            else if !t.is_completed  { 2 }
            else                     { 3 }
        };
        score(a).cmp(&score(b))
    });

    // --- Task rows ---
    let task_rows: Vec<Element<Message>> = sorted.iter().map(|task| {
        let difficulty_label = format!("[{}]", task.difficulty.as_str());

        let status_prefix = if task.is_pinned      { "📌 " }
            else if task.is_urgent { "🔴 " }
            else                   { "" };

        let title_str = format!("{}{} {}", status_prefix, task.title, difficulty_label);

        // Prerequisites lock indicator
        let prereq_blocked = !task.prerequisites.is_empty() && !task.prerequisites_met();
        let prereq_label: Element<Message> = if prereq_blocked {
            text("🔒 Prerequisites not met").size(12).into()
        } else if !task.prerequisites.is_empty() {
            text("✅ Prerequisites met").size(12).into()
        } else {
            Space::new().into()
        };

        // Category / due date metadata line
        let meta_parts: Vec<String> = {
            let mut parts = Vec::new();
            if !task.category.is_empty() {
                parts.push(format!("📁 {}", task.category));
            }
            if let Some(due_ms) = task.due_date {
                let due_secs = due_ms / 1000;
                if let Some(dt) = chrono::DateTime::from_timestamp(due_secs, 0) {
                    parts.push(format!("📅 {}", dt.format("%Y-%m-%d")));
                }
            }
            parts
        };
        let meta_line: Element<Message> = if meta_parts.is_empty() {
            Space::new().into()
        } else {
            text(meta_parts.join("   ")).size(12).into()
        };

        // Checkbox — disabled if prerequisites are not met
        let checkbox_widget: Element<Message> = if prereq_blocked {
            checkbox(task.is_completed).into()
        } else {
            checkbox(task.is_completed)
                .on_toggle(|_| Message::ToggleTask(task.id.clone()))
                .into()
        };

        column![
            row![
                checkbox_widget,
                column![
                    text(title_str).size(15),
                    meta_line,
                    prereq_label,
                ]
                .spacing(2),
                Space::new(),
            ]
            .spacing(12)
            .padding(6),
        ]
        .into()
    }).collect();

    let list: Element<Message> = if task_rows.is_empty() {
        container(
            text("No tasks yet. Add your first task to begin your quest!")
                .size(16),
        )
        .padding(20)
        .into()
    } else {
        scrollable(
            column(task_rows)
                .spacing(2)
                .padding(16)
                .width(Length::Fill),
        )
        .into()
    };

    column![form, list].into()
}