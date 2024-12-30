use bevy::prelude::*;

#[derive(Component)]
pub struct TextGroupSprite {}

// import letters texture atlas for letters
fn char_to_sprite_position(c: char) -> Vec2 {
    let (row, col) = match c {
        'A'..='J' => (0, c as u8 - b'A'),
        'K'..='T' => (1, c as u8 - b'K'),
        'U'..='Z' => (2, c as u8 - b'U'),
        ' ' => (2, 6),
        'a'..='j' => (3, c as u8 - b'a'),
        'k'..='t' => (4, c as u8 - b'k'),
        'u'..='z' => (5, c as u8 - b'u'),
        '0'..='9' => (6, c as u8 - b'0'),
        '.' => (7, 0),
        '?' => (7, 1),
        '!' => (7, 2),
        '"' => (7, 3),
        '\'' => (7, 4),
        '$' => (7, 5),
        '-' => (7, 6),
        '(' => (7, 7),
        ')' => (7, 8),
        '&' => (7, 9),
        _ => (7, 1),
    };
    Vec2::new(col as f32 * 6.0, row as f32 * 10.0)
}

// assemble &str to text sprites TODO: add a return type (maybe TextureAtlasBuilder,
// SpriteBindGroup)
pub fn spawn_text_sprite(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    position: Vec2,
) {
    for (i, c) in text.chars().enumerate() {
        if c == ' ' {}

        let pos = char_to_sprite_position(c);
        commands.spawn(SpriteBundle {
            texture: asset_server.load("alphabet.png"),
            sprite: Sprite {
                rect: Some(Rect::new(pos.x, pos.y, pos.x + 6.0, pos.y + 10.0)),
                custom_size: Some(Vec2::new(6.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(position.x + (i as f32 * 6.0), position.y, 100.0),
            ..default()
        });
    }
}
