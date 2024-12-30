use tracing::{event, Level};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MenuState {
    #[default]
    MainMenu,
    Playing,
    Paused,
}

#[derive(Component)]
pub struct Menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(Update, toggle_pause)
            .add_systems(OnEnter(MenuState::Paused), setup_pause_menu)
            .add_systems(OnExit(MenuState::Paused), cleanup_pause_menu)
            .add_systems(OnEnter(MenuState::MainMenu), setup_main_menu)
            .add_systems(OnExit(MenuState::MainMenu), cleanup_main_menu)
            .add_systems(Startup, startup);
    }
}

fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<MenuState>>,
    current_state: Res<State<MenuState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        event!(Level::INFO, "Toggling pause");
        match current_state.get() {
            MenuState::Playing => game_state.set(MenuState::Paused),
            MenuState::Paused => game_state.set(MenuState::Playing),
            _ => (),
        }
    }

    if keyboard.just_pressed(KeyCode::KeyM) {
        event!(Level::INFO, "Toggling menu");
        match current_state.get() {
            MenuState::MainMenu => game_state.set(MenuState::Playing),
            MenuState::Playing => game_state.set(MenuState::MainMenu),
            MenuState::Paused => game_state.set(MenuState::MainMenu),
        }
    }
}

fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PAUSED",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "MAIN MENU",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn startup(mut game_state: ResMut<NextState<MenuState>>) {
    game_state.set(MenuState::MainMenu); // Start in Playing state instead of MainMenu
}

pub fn pause_physics(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = false;
}

pub fn resume_physics(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = true;
}
