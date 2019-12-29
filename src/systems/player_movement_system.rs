use amethyst::{
    animation::{
        AnimationCommand,
        AnimationControlSet,
        ControlState,
    },
    ecs::{
        BitSet,
        Join,
        ReaderId,
        ReadExpect,
        ReadStorage,
        storage::ComponentEvent,
        System,
        WriteStorage,
    },
    renderer::SpriteRender,
};

use crate::{
    entities::player::{
        Direction,
        Player,
        PlayerAction,
        PlayerAnimation,
        PlayerSpriteSheets,
    },
};

pub struct PlayerMovementSystem {
    player_events_id: ReaderId<ComponentEvent>,
}

impl PlayerMovementSystem {
    pub fn new(storage: &mut WriteStorage<Player>) -> Self {
        PlayerMovementSystem {
            player_events_id: storage.register_reader(),
        }
    }
}

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, AnimationControlSet<PlayerAnimation, SpriteRender>>,
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
            match player.action {
                PlayerAction::Walk => player_walk(sprite_render, &sprite_sheets),
                PlayerAction::Run => player_run(sprite_render, &sprite_sheets),
            }

            let new_animation = get_new_animation(&player.action, &player.facing_direction);
            change_player_animation(&new_animation, control_set);
        }
    }
}

pub fn get_new_animation(action: &PlayerAction, direction: &Direction) -> PlayerAnimation {
    match (action, direction) {
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

pub fn player_walk(sprite_render: &mut SpriteRender, sprite_sheets: &PlayerSpriteSheets) {
    sprite_render.sprite_sheet = sprite_sheets.walking.clone();
    sprite_render.sprite_number = 0;
}

pub fn player_run(sprite_render: &mut SpriteRender, sprite_sheets: &PlayerSpriteSheets) {
    sprite_render.sprite_sheet = sprite_sheets.running.clone();
    sprite_render.sprite_number = 0;
}

fn change_player_animation(
    new_animation: &PlayerAnimation,
    control_set: &mut AnimationControlSet<PlayerAnimation, SpriteRender>,
) {
    control_set.animations
        .iter_mut()
        .filter(|(id, _)| *id != *new_animation)
        .for_each(|(_, animation)| {
            animation.command = AnimationCommand::Pause;
        });

    let (_, animation) = control_set.animations
        .iter_mut()
        .find(|(id, _)| *id == *new_animation)
        .unwrap();
    animation.state = ControlState::Requested;
    animation.command = AnimationCommand::Start;
}
