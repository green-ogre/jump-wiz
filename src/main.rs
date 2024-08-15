use avian2d::{dynamics::integrator::Gravity, PhysicsPlugins};
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    window::WindowResolution,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use map::MapPlugin;
use player::PlayerPlugin;

pub mod animated_sprites;
pub mod map;
pub mod player;

const GRAVITY: f32 = 2048.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "jump wiz".into(),
                        resolution: WindowResolution::new(map::WINDOW_SIZE, map::WINDOW_SIZE),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            PlayerPlugin,
            // PhysicsDebugPlugin::default(),
            // WorldInspectorPlugin::new(),
            MapPlugin,
        ))
        .add_systems(Update, close_on_escape)
        .insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
        .run();
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
