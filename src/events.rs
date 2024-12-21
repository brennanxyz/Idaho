use crate::{character::Character, player::Player};
use bevy::prelude::*;

#[derive(Component)]
pub struct FixedInteractionIndicator {
    pub trigger_distance: f32,
}

pub fn update_event_options(
    mut _commands: Commands,
    mut text_query: Query<(
        Entity,
        &Parent,
        &mut Visibility,
        &FixedInteractionIndicator,
    )>,
    player_query: Query<&Transform, With<Player>>,
    character_query: Query<&Transform, With<Character>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (_text_entity, parent, mut visibility, interaction_indicator) in text_query.iter_mut() {
            if let Ok(entity_transform) = character_query.get(parent.get()) {
                let distance = player_transform
                    .translation
                    .distance(entity_transform.translation);

                *visibility = if distance <= interaction_indicator.trigger_distance {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}

// BOOKMARK: need to find out how to spawn at fixed location.

pub enum EventType {
    Character,
    Location,
    Acquire,
}
pub struct EventPlugin;

pub fn spawn_fixed_sprite(
    commands: &mut Commands, 
    indicated_entity: Entity,
) {
    commands.entity(indicated_entity).with_children(|parent| {
        parent.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 5.0)),
                    color: Color::WHITE,                       
                    ..default()
                },
                transform: Transform::from_xyz(0.0, -20.0, 10.0),
                visibility: Visibility::Visible,
                ..default()
            },
            FixedInteractionIndicator {
                trigger_distance: 45.0,
            },
        ));
    });
}

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_event_options,
            ),
        );
    }
}