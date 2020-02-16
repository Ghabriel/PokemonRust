//! The systems present in the game that operate on its entities and resources.
//! Although a big part of the logic is actually encoded in events, there are
//! some important systems that help them do their job.

pub mod animation_system;
pub mod audio_system;
pub mod character_movement_system;
pub mod map_change_announcement_system;
pub mod npc_interaction_system;
pub mod player_input_system;
pub mod text_system;

pub use animation_system::AnimationSystem;
pub use audio_system::AudioSystem;
pub use character_movement_system::CharacterMovementSystem;
pub use map_change_announcement_system::MapChangeAnnouncementSystem;
pub use npc_interaction_system::NpcInteractionSystem;
pub use player_input_system::PlayerInputSystem;
pub use text_system::TextSystem;
