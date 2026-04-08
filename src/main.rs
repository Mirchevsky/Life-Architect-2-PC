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

            // v0.4 — initialise daily content engines
            data::daily_quote_engine::ensure_table(&conn)
                .expect("Failed to create daily_quote_state table");

            let daily_quote = data::daily_quote_engine::get_daily_quote(&conn);
            let today_event = data::global_events_engine::get_today_event();
            let tomorrow_title = data::global_events_engine::get_tomorrow_title();

            LifeArchitect::new(
                conn,
                tasks,
                goals,
                total_xp,
                daily_quote,
                today_event,
                tomorrow_title,
            )
        },
        LifeArchitect::update,
        LifeArchitect::view,
    )
    .title("Life Architect 2")
    .run()
}