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

const CHAR_WIDTH: f32 = 6.0;
const CHAR_HEIGHT: f32 = 10.0;
const CHARS_PER_ROW: f32 = 10.0;

fn get_char_rect(c: char) -> Rect {
    let (row, col) = match c {
        'A'..='J' => (0, c as u8 - b'A'),
        'K'..='T' => (1, c as u8 - b'K'),
        'U'..='Z' => (2, c as u8 - b'U'),
        'a'..='j' => (3, c as u8 - b'a'),
        'k'..='t' => (4, c as u8 - b'k'),
        'u'..='z' => (5, c as u8 - b'u'),
        '0'..='9' => (6, c as u8 - b'0'),
        _ => (0, 0), // Default to first character for unknown inputs
    };

    Rect::new(
        col as f32 * CHAR_WIDTH,
        row as f32 * CHAR_HEIGHT,
        (col as f32 * CHAR_WIDTH) + CHAR_WIDTH,
        (row as f32 * CHAR_HEIGHT) + CHAR_HEIGHT,
    )
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
            indicator.current_frame = (indicator.current_frame + 1) % indicator.animation_sequence.len();
            let frame_index = indicator.animation_sequence[indicator.current_frame];
            
            // Each frame is 3x3 pixels in a 9x3 sprite sheet
            sprite.rect = Some(Rect::new(
                frame_index as f32 * 5.0,
                0.0,
                (frame_index as f32 * 5.0) + 5.0,
                5.0
            ));
        }
    }
}

pub fn spawn_interaction_indicator(
    commands: &mut Commands,
    asset_server: &AssetServer,
    character_entity: Entity,
) {
    // commands.entity(character_entity).with_children(|parent| {
    //     parent.spawn((
    //         SpriteBundle {
    //             sprite: Sprite {
    //                 color: Color::BLACK,
    //                 custom_size: Some(Vec2::new(9.0, 12.0)), // TODO: find a way to add a border radius
    //                 ..default()
    //             },
    //             transform: Transform::from_xyz(0.0, 14.0, 0.0),
    //             visibility: Visibility::Hidden,
    //             ..default()
    //         },
    //         FloatingInteractionText {
    //             offset: Vec3::new(0.0, 12.0, 0.0),
    //             trigger_distance: 35.0,
    //             key_prompt: "E".to_string(),
    //         },
    //     ));
    //     parent.spawn((
    //         SpriteBundle {
    //             texture: asset_server.load("pixelorama/alphabet.png"),
    //             sprite: Sprite {
    //                 color: Color::WHITE,
    //                 custom_size: Some(Vec2::new(CHAR_WIDTH, CHAR_HEIGHT)),
    //                 rect: Some(get_char_rect('E')),
    //                 ..default()
    //             },
    //             transform: Transform::from_xyz(0.5, 13.0, 1.0),
    //             visibility: Visibility::Hidden,
    //             // text: Text::from_section(
    //             //     "E",
    //             //     TextStyle {
    //             //         font: asset_server.load("fonts/OpenSans-Regular.ttf"),
    //             //         font_size: 12.0,
    //             //         color: Color::WHITE,
    //             //     },
    //             // ),
    //             // transform: Transform::from_xyz(0.0, 14.0, 1.0),
    //             // visibility: Visibility::Hidden,
    //             ..default()
    //         },
    //         FloatingInteractionText {
    //             offset: Vec3::new(0.0, 14.0, 0.0),
    //             trigger_distance: 35.0,
    //             key_prompt: "E".to_string(),
    //         },
    //     ));
    // });
    commands.entity(character_entity).with_children(|parent| {
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
            FloatingInteractionText {
                offset: Vec3::new(0.0, 14.0, 0.0),
                trigger_distance: 35.0,
                key_prompt: "E".to_string(),
            },
            InteractionIndicator::default(),
        ));
    });
}

impl Plugin for MiscObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_interaction_indicators, animate_interaction_indicators));
    }
}