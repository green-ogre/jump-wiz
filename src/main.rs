use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    window::WindowResolution,
};
use bevy_ecs_ldtk::prelude::*;

const WINDOW_SIZE: f32 = 1000.0;
const TILE_SIZE: f32 = 512.0;
const TILE_MAP_SIZE: f32 = 16.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_SIZE, WINDOW_SIZE),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(LdtkPlugin)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .register_ldtk_int_cell_for_layer::<ColliderBundle>("Collision", 1)
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_escape, init_added_collision))
        .insert_resource(LevelSelection::index(0))
        .run();
}

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

#[derive(Clone, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    int_cell_collider: IntCellCollider,
    transform: Transform,
}

impl Default for ColliderBundle {
    fn default() -> Self {
        println!("crateing collider");
        Self {
            int_cell_collider: Default::default(),
            transform: Transform::default(),
        }
    }
}

#[derive(Default, Clone, Component)]
struct IntCellCollider;

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

fn init_added_collision(cells: Query<&GridCoords, Added<IntCellCollider>>) {
    for coords in cells.iter() {
        println!("{coords:?}");
    }
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
