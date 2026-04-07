use rusqlite::{Connection, Result};

/// One data point for the XP chart: a date label and the XP earned on that day.
#[derive(Debug, Clone)]
pub struct DailyXpPoint {
    pub label: String, // e.g. "Mon", "Tue" or "Apr 01"
    pub xp: u32,
}

/// Aggregated summary data for the top summary cards.
#[derive(Debug, Clone)]
pub struct AnalyticsSummary {
    pub total_xp: u32,
    pub tasks_completed: u32,
    pub tasks_pending: u32,
    pub xp_last_7_days: Vec<DailyXpPoint>,
    pub xp_last_30_days: Vec<DailyXpPoint>,
}

/// Queries the database and returns a fully populated AnalyticsSummary.
pub fn get_analytics_summary(conn: &Connection) -> Result<AnalyticsSummary> {
    let total_xp = get_total_xp(conn)?;
    let tasks_completed = count_tasks(conn, true)?;
    let tasks_pending = count_tasks(conn, false)?;
    let xp_last_7_days = get_daily_xp(conn, 7)?;
    let xp_last_30_days = get_daily_xp(conn, 30)?;

    Ok(AnalyticsSummary {
        total_xp,
        tasks_completed,
        tasks_pending,
        xp_last_7_days,
        xp_last_30_days,
    })
}

fn get_total_xp(conn: &Connection) -> Result<u32> {
    conn.query_row(
        "SELECT total_xp FROM user_progress WHERE id = 1",
        [],
        |row| row.get::<_, u32>(0),
    )
}

fn count_tasks(conn: &Connection, completed: bool) -> Result<u32> {
    conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE is_completed = ?1",
        rusqlite::params![completed as i32],
        |row| row.get::<_, u32>(0),
    )
}

/// Returns daily XP earned over the last `days` days, one entry per day.
/// Days with no completions are included as 0 XP so the chart has a full
/// continuous series.
fn get_daily_xp(conn: &Connection, days: i64) -> Result<Vec<DailyXpPoint>> {
    use chrono::{Duration, Local, NaiveDate, TimeZone};

    let today = Local::now().date_naive();
    let mut points: Vec<DailyXpPoint> = Vec::with_capacity(days as usize);

    // Build a map of date -> xp from the database
    let cutoff_ms = Local
        .from_local_datetime(
            &(today - Duration::days(days - 1))
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .unwrap()
        .timestamp_millis();

    let mut stmt = conn.prepare(
        "SELECT completed_at, difficulty FROM tasks
         WHERE is_completed = 1 AND completed_at IS NOT NULL AND completed_at >= ?1",
    )?;

    let rows = stmt.query_map(rusqlite::params![cutoff_ms], |row| {
        let completed_at: i64 = row.get(0)?;
        let difficulty: String = row.get(1)?;
        Ok((completed_at, difficulty))
    })?;

    // Accumulate XP per date
    let mut xp_map: std::collections::HashMap<NaiveDate, u32> =
        std::collections::HashMap::new();

    for row in rows.filter_map(|r| r.ok()) {
        let (ts_ms, difficulty_str) = row;
        let xp: u32 = match difficulty_str.as_str() {
            "Easy" => 10,
            "Hard" => 50,
            "Epic" => 100,
            _ => 25, // Medium
        };
        let date = Local
            .timestamp_millis_opt(ts_ms)
            .unwrap()
            .date_naive();
        *xp_map.entry(date).or_insert(0) += xp;
    }

    // Build the ordered series, filling gaps with 0
    for i in 0..days {
        let date: NaiveDate = today - Duration::days(days - 1 - i);
        let xp = *xp_map.get(&date).unwrap_or(&0);
        let label = if days <= 7 {
            // Short label: day-of-week abbreviation
            date.format("%a").to_string()
        } else {
            // Longer label: "Apr 01"
            date.format("%b %d").to_string()
        };
        points.push(DailyXpPoint { label, xp });
    }

    Ok(points)
}