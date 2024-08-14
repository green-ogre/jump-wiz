use avian2d::{debug_render::PhysicsDebugPlugin, dynamics::integrator::Gravity, PhysicsPlugins};
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    window::WindowResolution,
};
use bevy_ecs_ldtk::prelude::*;

use player::PlayerPlugin;

pub mod animated_sprites;
pub mod player;

const WINDOW_SIZE: f32 = 1000.0;
const TILE_SIZE: f32 = 512.0;
const TILE_MAP_SIZE: f32 = 16.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "jump wiz".into(),
                    resolution: WindowResolution::new(WINDOW_SIZE, WINDOW_SIZE),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            PhysicsPlugins::default(),
            PlayerPlugin,
            PhysicsDebugPlugin::default(),
            LdtkPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_escape)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(Gravity(Vec2::NEG_Y * 8192.))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = (TILE_SIZE * TILE_MAP_SIZE / 2.0) / (WINDOW_SIZE);
    camera.transform.translation.x = WINDOW_SIZE / 2.0 * camera.projection.scale;
    camera.transform.translation.y = WINDOW_SIZE / 2.0 * camera.projection.scale;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk"),
        ..Default::default()
    });
}

fn close_on_escape(mut input: EventReader<KeyboardInput>, mut writer: EventWriter<AppExit>) {
    for e in input.read() {
        if matches!(e, KeyboardInput {

            key_code,
            state,
            ..
        }

            if *key_code == KeyCode::Escape && *state == ButtonState::Pressed
        ) {
            writer.send(AppExit::Success);
        }
    }
}
