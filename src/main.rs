mod app;
mod app_core;
mod data;
mod ui;
mod utils;

use app::LifeArchitect;

fn main() -> iced::Result {
    iced::application(
        || {
            let conn = data::database::open_database()
                .expect("Failed to open database");
            let tasks = data::task_dao::get_all_tasks(&conn)
                .unwrap_or_default();
            let goals = data::goal_dao::get_all_goals(&conn)
                .unwrap_or_default();
            let total_xp = data::task_dao::get_total_xp(&conn)
                .unwrap_or(0);
            LifeArchitect::new(conn, tasks, goals, total_xp)
        },
        LifeArchitect::update,
        LifeArchitect::view,
    )
    .title("Life Architect 2")
    .run()
}