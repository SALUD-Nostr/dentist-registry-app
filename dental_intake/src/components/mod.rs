//! UI Components for salud-dental
//!
//! App-specific components. General components (Card, Button, etc.) come from shady-minions

pub mod button;
pub mod icons;
pub mod logo;
pub mod typography;

// Re-export commonly used components
pub use button::{Button, ButtonSize, ButtonVariant};
pub use icons::*;
pub use logo::Logo;
pub use typography::*;
