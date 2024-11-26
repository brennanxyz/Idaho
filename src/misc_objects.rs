use crate::{player::Player, character::Character};
use bevy::prelude::*;
// use bevy_ecs_ldtk::prelude::*;

// use crate::colliders::ColliderBundle;

// #[derive(Clone, Default, Bundle, LdtkEntity)]
// pub struct ChestBundle {
//     #[sprite_sheet_bundle]
//     pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
//     #[from_entity_instance]
//     pub collider_bundle: ColliderBundle,
// }

// #[derive(Clone, Default, Bundle, LdtkEntity)]
// pub struct PumpkinsBundle {
//     #[sprite_sheet_bundle(no_grid)]
//     pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
// }

pub struct MiscObjectsPlugin;

// impl Plugin for MiscObjectsPlugin {
//     fn build(&self, app: &mut App) {
//         app.register_ldtk_entity::<ChestBundle>("Chest")
//             .register_ldtk_entity::<PumpkinsBundle>("Pumpkins");
//     }
// }

#[derive(Component)]
pub struct FloatingInteractionText {
    pub offset: Vec3,
    pub trigger_distance: f32,
    pub key_prompt: String,
}

pub fn update_interaction_indicators(
    mut commands: Commands,
    mut text_query: Query<(Entity, &Parent, &mut Visibility, &FloatingInteractionText)>,
    player_query: Query<&Transform, With<Player>>,
    character_query: Query<&Transform, With<Character>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (text_entity, parent, mut visibility, interaction_text) in text_query.iter_mut() {
            if let Ok(character_transform) = character_query.get(parent.get()) {
                let distance = player_transform.translation.distance(character_transform.translation);
                
                *visibility = if distance <= interaction_text.trigger_distance {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}

pub fn spawn_interaction_indicator(
    commands: &mut Commands,
    asset_server: &AssetServer,
    character_entity: Entity,
) {
    commands.entity(character_entity).with_children(|parent| {
        parent.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(12.0, 12.0)), // TODO: find a way to add a border radius
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 14.0, 0.0),
                visibility: Visibility::Hidden,
                ..default()
            },
            FloatingInteractionText {
                offset: Vec3::new(0.0, 12.0, 0.0),
                trigger_distance: 35.0,
                key_prompt: "E".to_string(),
            },
        ));
        parent.spawn((
            Text2dBundle {
                text: Text::from_section(
                    "E",
                    TextStyle {
                        font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                        font_size: 12.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(0.0, 14.0, 1.0),
                visibility: Visibility::Hidden,
                ..default()
            },
            FloatingInteractionText {
                offset: Vec3::new(0.0, 14.0, 0.0),
                trigger_distance: 35.0,
                key_prompt: "E".to_string(),
            },
        ));
    });
}

impl Plugin for MiscObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_interaction_indicators);
    }
}