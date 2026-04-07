/// Calculates the user's level from their total XP.
pub fn calculate_level(total_xp: u32) -> u32 {
    (1.0 + (total_xp as f64 / 50.0).sqrt()) as u32
}

/// Calculates the XP needed to reach the next level.
pub fn xp_for_next_level(current_level: u32) -> u32 {
    let next_level = current_level + 1;
    50 * (next_level - 1).pow(2)
}

/// Calculates XP accumulated within the current level (for the progress bar).
pub fn xp_within_current_level(total_xp: u32) -> u32 {
    let current_level = calculate_level(total_xp);
    let xp_at_current_level_start = 50 * (current_level - 1).pow(2);
    total_xp.saturating_sub(xp_at_current_level_start)
}