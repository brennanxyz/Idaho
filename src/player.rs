use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::{climbing::Climber, inventory::Inventory};
use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};

#[derive(Eq, PartialEq)]
pub enum CharacterDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    LAST,
}

#[derive(Resource)]
pub struct AnimationTimer {
    pub timer: Timer,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle("main_char_sheet.png", 20, 17, 4, 8, 0, 0, 0)]
    pub sprite_bundle: LdtkSpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub climber: Climber,
    pub ground_detection: GroundDetection,

    // Build Items Component manually by using `impl From<&EntityInstance>`
    #[from_entity_instance]
    items: Inventory,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut config: ResMut<AnimationTimer>,
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection, &mut TextureAtlas), With<Player>>,
) {
    for (mut velocity, mut climber, ground_detection, mut tas) in &mut query {
        // let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
        // let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };
        let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
        let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };
        let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

        let current_direction = match (up, down, right, left) {
            (1., 0., 0., 0.) => CharacterDirection::N,
            (1., 0., 1., 0.) => CharacterDirection::NE,
            (0., 0., 1., 0.) => CharacterDirection::E,
            (0., 1., 1., 0.) => CharacterDirection::SE,
            (0., 1., 0., 0.) => CharacterDirection::S,
            (0., 1., 0., 1.) => CharacterDirection::SW,
            (0., 0., 0., 1.) => CharacterDirection::W,
            (1., 0., 0., 1.) => CharacterDirection::NW,
            _ => CharacterDirection::LAST,
        };

        

        config.timer.tick(time.delta());
        let update: bool = config.timer.just_finished();

        match &current_direction {
            CharacterDirection::N => {
                animate(16, 19, &mut tas, &update);
            },
            CharacterDirection::NE => {
                animate(20, 23, &mut tas, &update);
            },
            CharacterDirection::E => {
                animate(24, 27, &mut tas, &update);
            },
            CharacterDirection::SE => {
                animate(28, 31, &mut tas, &update);
            },
            CharacterDirection::S => {
                animate(0, 3, &mut tas, &update);
            },
            CharacterDirection::SW => {
                animate(4, 7, &mut tas, &update);
            },
            CharacterDirection::W => {
                animate(8, 11, &mut tas, &update);
            },
            CharacterDirection::NW => {
                animate(12, 15, &mut tas, &update);
            },
            _ => {
                match tas.index {
                    16..=19 => tas.index = 16,
                    20..=23 => tas.index = 20,
                    24..=27 => tas.index = 24,
                    28..=31 => tas.index = 28,
                    0..=3 => tas.index = 0,
                    4..=7 => tas.index = 4,
                    8..=11 => tas.index = 8,
                    12..=15 => tas.index = 12,
                    _ => (),
                }
            },
        }

        velocity.linvel.x = (right - left) * 100.;
        velocity.linvel.y = (up - down) * 100.;

        // velocity.linvel.x = (right - left) * 200.;

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::KeyS) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };

            velocity.linvel.y = (up - down) * 200.;
        }

        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || climber.climbing) {
            velocity.linvel.y = 500.;
            climber.climbing = false;
        }
    }
}

fn animate(start_frame: usize, end_frame: usize, tas: &mut TextureAtlas, update: &bool) {
    if (tas.index < start_frame || tas.index > end_frame || tas.index == end_frame) && update == &true {
        tas.index = start_frame;
    } else if update == &true {
        tas.index += 1;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}
