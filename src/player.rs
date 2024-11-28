use tracing::{event, Level};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::{climbing::Climber, inventory::Inventory};
use crate::colliders::ColliderBundle;

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

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    pub worldly: Worldly,
    pub climber: Climber,
    pub inventory: Inventory,
    entity_instance: EntityInstance,
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> PlayerBundle {
        let backup = "main_char_sheet.png".to_string();
        let sprite_sheet = LdtkFields::get_string_field(entity_instance, "sprite_sheet")
            .unwrap_or(&backup);
        let layout = TextureAtlasLayout::from_grid(UVec2::new(20, 17), 4, 8, None, None);
        let atlas_layout = texture_atlases.add(layout);

        PlayerBundle {
            sprite_sheet_bundle: LdtkSpriteSheetBundle {
                sprite_bundle: SpriteBundle {
                    texture: asset_server.load(sprite_sheet),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                },
                texture_atlas: atlas_layout.into(),
                ..default()
            },
            collider_bundle: ColliderBundle::from(entity_instance),
            player: Player,
            worldly: Worldly::from_entity_info(entity_instance),
            climber: Climber::default(),
            inventory: Inventory::default(),
            entity_instance: entity_instance.clone(),
            // velocity: Velocity::default(),
        }
    }
}

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut config: ResMut<AnimationTimer>,
    mut query: Query<(&mut Velocity, &mut Climber, &mut TextureAtlas), With<Player>>,
) {
    for (mut velocity, mut climber, mut tas) in &mut query {
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

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::KeyS) {
            climber.climbing = true;
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
        event!(Level::INFO, "Player plugin registered");
    }
}
