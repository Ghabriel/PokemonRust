use amethyst::{
    animation::AnimationControlSet,
    ecs::{
        BitSet,
        Join,
        ReaderId,
        ReadExpect,
        ReadStorage,
        storage::ComponentEvent,
        System,
        WriteStorage,
        World,
        WorldExt,
    },
    renderer::SpriteRender,
};

use crate::{
    common::Direction,
    entities::{
        CharacterAnimation,
        change_character_animation,
        player::{Player, PlayerAction, PlayerAnimation, PlayerSpriteSheets},
    },
};

pub struct PlayerAnimationSystem {
    player_events_id: ReaderId<ComponentEvent>,
}

impl PlayerAnimationSystem {
    pub fn new(world: &mut World) -> Self {
        PlayerAnimationSystem {
            player_events_id: world.write_storage::<Player>().register_reader(),
        }
    }
}

impl<'a> System<'a> for PlayerAnimationSystem {
    type SystemData = (
        WriteStorage<'a, AnimationControlSet<CharacterAnimation, SpriteRender>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, SpriteRender>,
        ReadExpect<'a, PlayerSpriteSheets>,
    );

    fn run(&mut self, (
        mut control_sets,
        players,
        mut sprite_renders,
        sprite_sheets,
    ): Self::SystemData) {
        let mut modified = BitSet::new();

        for event in players.channel().read(&mut self.player_events_id) {
            match event {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    modified.add(*id);
                },
                ComponentEvent::Removed(_) => {},
            }
        }

        for (
            control_set,
            player,
            sprite_render,
            _,
        ) in (&mut control_sets, &players, &mut sprite_renders, &modified).join() {
            let actual_action = if player.moving {
                player.action.clone()
            } else {
                PlayerAction::Idle
            };

            match actual_action {
                PlayerAction::Idle | PlayerAction::Walk => {
                    sprite_render.sprite_sheet = sprite_sheets.walking.clone();
                },
                PlayerAction::Run => sprite_render.sprite_sheet = sprite_sheets.running.clone(),
            }

            let new_animation = get_new_animation(&actual_action, &player.facing_direction);
            change_character_animation(new_animation.into(), control_set);
        }
    }
}

pub fn get_new_animation(action: &PlayerAction, direction: &Direction) -> PlayerAnimation {
    match (action, direction) {
        (PlayerAction::Idle, Direction::Up) => PlayerAnimation::IdleUp,
        (PlayerAction::Idle, Direction::Down) => PlayerAnimation::IdleDown,
        (PlayerAction::Idle, Direction::Left) => PlayerAnimation::IdleLeft,
        (PlayerAction::Idle, Direction::Right) => PlayerAnimation::IdleRight,
        (PlayerAction::Walk, Direction::Up) => PlayerAnimation::WalkUp,
        (PlayerAction::Walk, Direction::Down) => PlayerAnimation::WalkDown,
        (PlayerAction::Walk, Direction::Left) => PlayerAnimation::WalkLeft,
        (PlayerAction::Walk, Direction::Right) => PlayerAnimation::WalkRight,
        (PlayerAction::Run, Direction::Up) => PlayerAnimation::RunUp,
        (PlayerAction::Run, Direction::Down) => PlayerAnimation::RunDown,
        (PlayerAction::Run, Direction::Left) => PlayerAnimation::RunLeft,
        (PlayerAction::Run, Direction::Right) => PlayerAnimation::RunRight,
    }
}
