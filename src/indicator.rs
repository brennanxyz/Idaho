use crate::{character::Character, player::Player};
use bevy::prelude::*;

pub struct IndicatorPlugin;

#[derive(Component)]
pub struct FloatingInteractionIndicator {
    pub trigger_distance: f32,
}

pub fn update_interaction_indicators(
    mut _commands: Commands,
    mut text_query: Query<(
        Entity,
        &Parent,
        &mut Visibility,
        &FloatingInteractionIndicator,
    )>,
    player_query: Query<&Transform, With<Player>>,
    character_query: Query<&Transform, With<Character>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (_text_entity, parent, mut visibility, interaction_indicator) in text_query.iter_mut() {
            if let Ok(character_transform) = character_query.get(parent.get()) {
                let distance = player_transform
                    .translation
                    .distance(character_transform.translation);

                *visibility = if distance <= interaction_indicator.trigger_distance {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}

#[derive(Component)]
pub struct InteractionIndicator {
    pub timer: Timer,
    pub current_frame: usize,
    pub animation_sequence: Vec<usize>,
}

impl Default for InteractionIndicator {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            current_frame: 0,
            animation_sequence: vec![0, 1, 2, 3],
        }
    }
}

fn animate_interaction_indicators(
    time: Res<Time>,
    mut query: Query<(&mut InteractionIndicator, &mut Sprite)>,
) {
    for (mut indicator, mut sprite) in query.iter_mut() {
        indicator.timer.tick(time.delta());
        if indicator.timer.just_finished() {
            indicator.current_frame =
                (indicator.current_frame + 1) % indicator.animation_sequence.len();
            let frame_index = indicator.animation_sequence[indicator.current_frame];

            // Each frame is 3x3 pixels in a 9x3 sprite sheet
            sprite.rect = Some(Rect::new(
                frame_index as f32 * 5.0,
                0.0,
                (frame_index as f32 * 5.0) + 5.0,
                5.0,
            ));
        }
    }
}

pub fn spawn_interaction_indicator(
    commands: &mut Commands,
    asset_server: &AssetServer,
    indicated_entity: Entity,
) {
    commands.entity(indicated_entity).with_children(|parent| {
        parent.spawn((
            SpriteBundle {
                texture: asset_server.load("pixelorama/indicator_blink.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    rect: Some(Rect::new(0.0, 0.0, 5.0, 5.0)), // Start with first frame
                    ..default()
                },
                transform: Transform::from_xyz(8.0, 8.0, 1.0), // Offset to right and above character
                visibility: Visibility::Hidden,
                ..default()
            },
            FloatingInteractionIndicator {
                trigger_distance: 45.0,
            },
            InteractionIndicator::default(),
        ));
    });
}

impl Plugin for IndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_interaction_indicators,
                animate_interaction_indicators,
            ),
        );
    }
}
