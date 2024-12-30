use crate::{character::Character, player::Player, text::spawn_text_sprite};
use bevy::prelude::*;

pub struct EventPlugin;

#[derive(Component)]
pub struct FixedEventIndicator {
    pub trigger_distance: f32,
}

pub fn update_event_indicators(
    mut _commands: Commands,
    mut node_query: Query<(&mut Visibility, &FixedEventIndicator)>,
    player_query: Query<&Transform, With<Player>>,
    character_query: Query<&Transform, With<Character>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for character_transform in character_query.iter() {
            for (mut visibility, interaction_indicator) in node_query.iter_mut() {
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

// BOOKMARK: need to find out how to spawn at fixed location.

pub enum EventType {
    Character,
    Location,
    Acquire,
}

pub fn spawn_event_indicator_sprite(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    spawn_text_sprite(
        commands,
        &asset_server,
        "Sphinx of black quartz, judge my vow.",
        Vec2::new(50.0, 50.0),
    );
    //commands.spawn((
    //    NodeBundle {
    //        style: Style {
    //            width: Val::Percent(100.0),
    //            height: Val::Percent(100.0),
    //            flex_direction: FlexDirection::Column,
    //            align_items: AlignItems::Start,
    //            justify_content: JustifyContent::End,
    //            ..default()
    //        },
    //        // background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
    //        visibility: Visibility::Hidden,
    //        ..default()
    //    },
    //    FixedEventIndicator {
    //        trigger_distance: 45.0,
    //    },
    //
    //))
    //.with_children(|parent | {
    //    parent.spawn(
    //        words
    //    );
    //});
}

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_event_indicators,));
    }
}
