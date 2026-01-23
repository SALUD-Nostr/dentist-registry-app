//! Utility functions for sorting and data manipulation

use salud_types::Encounter;

/// Sort encounters by date descending (most recent first)
pub fn sort_encounters_by_date(mut encounters: Vec<Encounter>) -> Vec<Encounter> {
    encounters.sort_by(|a, b| {
        let date_a = a
            .period
            .as_ref()
            .and_then(|p| p.start)
            .unwrap_or_else(chrono::Utc::now);
        let date_b = b
            .period
            .as_ref()
            .and_then(|p| p.start)
            .unwrap_or_else(chrono::Utc::now);
        date_b.cmp(&date_a)
    });
    encounters
}
