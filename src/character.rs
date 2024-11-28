use crate::{
    colliders::ColliderBundle,
    misc_objects::spawn_interaction_indicator,
};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Character;

#[derive(Clone, Default, Bundle)]
pub struct CharacterBundle {
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    pub collider_bundle: ColliderBundle,
    pub character: Character,
    pub worldly: Worldly,
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
            worldly: Worldly::from_entity_info(entity_instance),
        }
    }
}

fn spawn_characters(
    mut commands: Commands,
    character_query: Query<Entity, Added<Character>>,
    asset_server: Res<AssetServer>,
) {
    for entity in character_query.iter() {
        spawn_interaction_indicator(&mut commands, &asset_server, entity);
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CharacterBundle>("Character")
            .add_systems(Update, spawn_characters);
    }
}