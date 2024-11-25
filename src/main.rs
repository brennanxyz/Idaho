// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

use bevy::prelude::*;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

mod components;
mod systems;

fn main() {
    // let mut window_plugin = WindowPlugin::default();
    // window_plugin.primary_window = Some(Window {
    //     title: "Game".to_string(),
    //     canvas: Some("#game_canvas".to_string()),
    //     ..Default::default()
    // });
    // let w_plugins = DefaultPlugins.set(window_plugin);
    // w_plugins.add(ImagePlugin::default_nearest());

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugins(w_plugins)
        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Required to prevent race conditions between bevy_ecs_ldtk's and bevy_rapier's systems
        //.configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
        .insert_resource(RapierConfiguration {
            // gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_systems(Startup, systems::setup)
        .add_systems(Update, systems::spawn_wall_collision)
        .add_systems(Update, systems::movement)
        .add_systems(Update, systems::camera_fit_inside_current_level)
        .add_systems(Update, systems::update_level_selection)
        .add_systems(Update, systems::dbg_player_items)
        .add_systems(Update, systems::restart_level)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::LadderBundle>(2)
        .register_ldtk_int_cell::<components::WallBundle>(3)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        //.register_ldtk_entity::<components::MobBundle>("Mob")
        //.register_ldtk_entity::<components::ChestBundle>("Chest")
        //.register_ldtk_entity::<components::PumpkinsBundle>("Pumpkins")
        .run();
}
