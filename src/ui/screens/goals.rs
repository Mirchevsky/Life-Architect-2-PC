use crate::app::Message;
use crate::app_core::models::{Goal, Task};
use iced::widget::{button, column, container, progress_bar, row, scrollable, text, text_input, Space};
use iced::{Element, Length};

pub fn view<'a>(
    goals: &'a [Goal],
    tasks: &'a [Task],
    input_value: &'a str,
) -> Element<'a, Message> {
    // --- Input row ---
    let input_row = row![
        text_input("New goal title...", input_value)
            .on_input(Message::NewGoalInputChanged)
            .on_submit(Message::AddGoal)
            .padding(10)
            .width(Length::Fill),
        button("Add Goal")
            .on_press(Message::AddGoal)
            .padding(10),
    ]
    .spacing(8)
    .padding(16);

    // --- Goal cards ---
    let goal_cards: Vec<Element<Message>> = goals.iter().map(|goal| {
        let goal_tasks: Vec<&Task> = tasks.iter()
            .filter(|t| t.goal_id == goal.id)
            .collect();

        let total = goal_tasks.len();
        let completed = goal_tasks.iter().filter(|t| t.is_completed).count();

        let progress = if total > 0 {
            completed as f32 / total as f32
        } else {
            0.0
        };

        let progress_label = if total == 0 {
            "No tasks assigned yet".to_string()
        } else {
            format!("{} / {} tasks completed", completed, total)
        };

        let card_content = column![
            row![
                text(&goal.title).size(18),
                Space::new(),
                button("Delete")
                    .on_press(Message::DeleteGoal(goal.id.clone()))
                    .padding([4, 10]),
            ]
            .spacing(8),
            text(progress_label).size(13),
            progress_bar(0.0..=1.0, progress),
        ]
        .spacing(6)
        .padding(14)
        .width(Length::Fill);

        container(card_content)
            .width(Length::Fill)
            .into()
    }).collect();

    let list: Element<Message> = if goal_cards.is_empty() {
        container(
            text("No goals yet. Create a goal to group your tasks into a larger objective.")
                .size(15),
        )
        .padding(20)
        .into()
    } else {
        scrollable(
            column(goal_cards)
                .spacing(8)
                .padding(16)
                .width(Length::Fill),
        )
        .into()
    };

    column![input_row, list].into()
}