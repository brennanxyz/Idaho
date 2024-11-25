use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
// use bevy_rapier2d::dynamics::Velocity;

use crate::colliders::ColliderBundle;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Character;

#[derive(Clone, Default, Bundle)]
pub struct CharacterBundle {
    // #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    // #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub character: Character,
    // #[worldly]
    pub worldly: Worldly,
    // pub velocity: Velocity,
}

impl LdtkEntity for CharacterBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> CharacterBundle {
        let backup = "main_char_sheet.png".to_string();
        let sprite_sheet = LdtkFields::get_string_field(entity_instance, "sprite_sheet")
            .unwrap_or(&backup);
        let layout = TextureAtlasLayout::from_grid(UVec2::new(20, 17), 4, 8, None, None);
        let atlas_layout = texture_atlases.add(layout);

        CharacterBundle {
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
            character: Character,
            worldly: Worldly::default(),
            // velocity: Velocity::default(),
        }
    }
}

// impl From<&EntityInstance> for CharacterBundle {
//     fn from(entity_instance: &EntityInstance) -> CharacterBundle {
//         let sprite_sheet = entity_instance.get_string_field("sprite_sheet");
        
//         CharacterBundle {
//             sprite_bundle: LdtkSpriteSheetBundle {
//                 sprite_sheet_bundle: SpriteSheetBundle {
//                     texture_atlas: asset_server.load(sprite_sheet),
//                     transform: Transform::from_xyz(0.0, 0.0, 0.0),
//                     ..default()
//                 },
//                 ..default()
//             },
//             collider_bundle: ColliderBundle::from(entity_instance),
//             character: Character,
//             worldly: Worldly::from(entity_instance),
//         }
//     }
// }

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CharacterBundle>("Character");
    }
}