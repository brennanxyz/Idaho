use tracing::{event, Level};
use tracing_subscriber::fmt::writer::MakeWriterExt;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_common_assets::xml::XmlAssetPlugin;

mod camera;
mod character;
mod climbing;
/// Bundles for auto-loading Rapier colliders as part of the level
mod colliders;
mod enemy;
/// Handles initialization and switching levels
mod game_flow;
mod inventory;
mod menu;
mod misc_objects;
mod player;
mod timeline;
mod walls;

fn main() {
    // Set up logging
    let logfile = tracing_appender::rolling::hourly("./logs", "prefix.log");
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);
    tracing_subscriber::fmt()
        .pretty()
        .with_writer(stdout.and(logfile))
        .init();
    event!(Level::INFO, "Launching...");

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            },
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(game_flow::GameFlowPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(climbing::ClimbingPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(character::CharacterPlugin)
        .add_plugins(misc_objects::MiscObjectsPlugin)
        .add_plugins(menu::MenuPlugin)

        .add_plugins(XmlAssetPlugin::<timeline::Timeline>::new(&["timelines/timeline.xml"]))

        .add_systems(Update, inventory::dbg_print_inventory)
        .add_systems(Update, camera::camera_fit_inside_current_level)
        .add_systems(Update, menu::pause_physics.run_if(in_state(menu::GameState::Paused)))
        .add_systems(OnEnter(menu::GameState::Playing), menu::resume_physics)
        
        .run();
}
