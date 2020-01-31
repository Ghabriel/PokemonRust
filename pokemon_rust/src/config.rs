//! Types related to the main configuration file (`settings.ron`).

use serde::{Deserialize, Serialize};

/// Describes the available fields of `settings.ron`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
    /// The speed of the player when they're walking, in pixels/second.
    pub player_walking_speed: f32,
    /// The speed of the player when they're running, in pixels/second.
    pub player_running_speed: f32,
    /// The delay between letters appearing within a text box, in
    /// characters/second. This can be less than the interval between frames:
    /// that results in multiple characters appearing per frame.
    pub text_delay: f32,
    /// The duration of fade in/out animations, in seconds.
    pub fade_duration: f32,
    /// The starting position of the player, in Map Coordinates.
    pub player_starting_position: (u32, u32),
    /// Decides whether the FPS should be printed
    pub show_fps: bool,
}
