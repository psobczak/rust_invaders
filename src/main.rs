use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use rust_invaders::{
    Cell, GameState, Grid, InvaderPlugin, MyAssets, PlayerPlugin, ProjectilePlugin, ScorePlugin,
    SpaceshipPlugin,
};

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Next),
        )
        .add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Invaders".into(),
                resolution: WindowResolution::new(480.0, 600.0),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Slash)),
        )
        .add_plugin(InvaderPlugin)
        .add_plugin(SpaceshipPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(ScorePlugin)
        .add_system(setup.on_startup())
        .run();
}

fn setup(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2dBundle::default());

    let window = window.single();
    let grid = Grid {
        rows: (window.height() / Cell::SIZE) as isize,
        columns: (window.width() / Cell::SIZE) as isize,
    };

    commands.insert_resource(grid);
}
