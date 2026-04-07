use crate::app::Message;
use crate::app_core::models::Task;
use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input};
use iced::{Element, Length};

pub fn view<'a>(tasks: &'a [Task], input_value: &'a str) -> Element<'a, Message> {
    // Input row — Enter key submits via on_submit
    let input_row = row![
        text_input("Type a task title and press Enter...", input_value)
            .on_input(Message::NewTaskInputChanged)
            .on_submit(Message::AddTask)
            .padding(10)
            .width(Length::Fill),
        button("Add Task")
            .on_press(Message::AddTask)
            .padding(10),
    ]
    .spacing(8)
    .padding(16);

    // Sort: pinned first, then urgent, then pending, then completed
    let mut sorted: Vec<&Task> = tasks.iter().collect();
    sorted.sort_by(|a, b| {
        let score = |t: &&Task| -> u8 {
            if t.is_pinned { 0 }
            else if t.is_urgent { 1 }
            else if !t.is_completed { 2 }
            else { 3 }
        };
        score(a).cmp(&score(b))
    });

    let task_rows: Vec<Element<Message>> = sorted.iter().map(|task| {
        let difficulty_label = format!("[{}]", task.difficulty.as_str());
        let status_prefix = if task.is_pinned { "📌 " }
            else if task.is_urgent { "🔴 " }
            else { "" };
        let title = format!("{}{} {}", status_prefix, task.title, difficulty_label);

        row![
            checkbox(task.is_completed)
                .on_toggle(|_| Message::ToggleTask(task.id.clone())),
            text(title).size(15),
        ]
        .spacing(12)
        .padding(6)
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

    column![input_row, list].into()
}