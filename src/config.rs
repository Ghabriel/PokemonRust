use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
    pub player_walking_speed: f32,
    pub player_running_speed: f32,
}
