use avian2d::PhysicsPlugins;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResolution,
};
use maze::MazePlugin;
use player::PlayerPlugin;

pub mod animated_sprites;
pub mod maze;
pub mod player;

fn main() {
    App::default()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("maze-lite"),
                        resolution: WindowResolution::new(1200.0, 1200.0),
                        ..Default::default()
                    }),
                    ..default()
                })
                // for crisp sprites if we want that
                .set(ImagePlugin::default_nearest()),
            MazePlugin,
            PhysicsPlugins::default(),
            PlayerPlugin,
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .insert_resource(avian2d::prelude::Gravity::ZERO)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
