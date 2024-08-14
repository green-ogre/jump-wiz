use avian2d::{
    collision::Collider,
    debug_render::PhysicsDebugPlugin,
    dynamics::{integrator::Gravity, rigid_body::RigidBody},
    PhysicsPlugins,
};
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    window::WindowResolution,
};
use bevy_ecs_ldtk::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_input_manager::InputManagerBundle;
use player::{
    input::PlayerActionSidescroller, movement::CharacterControllerBundle, Player, PlayerPlugin,
};

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
        .add_plugins(WorldInspectorPlugin::new())
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .register_ldtk_int_cell_for_layer::<ColliderBundle>("Collision", 1)
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_escape, init_added_collision))
        .insert_resource(LevelSelection::index(0))
        .insert_resource(Gravity(Vec2::NEG_Y * 8192.))
        .run();
}

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    player: Player,
    movement: CharacterControllerBundle,
    input: InputManagerBundle<PlayerActionSidescroller>,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            movement: CharacterControllerBundle::default(),
            input: InputManagerBundle::with_map(PlayerActionSidescroller::default_input_map()),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
        }
    }
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
        Self {
            int_cell_collider: Default::default(),
            transform: Transform::default(),
        }
    }
}

#[derive(Default, Clone, Component)]
struct IntCellCollider;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk"),
        transform: Transform::default()
            .with_scale(Vec3::new(
                WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0),
                WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0),
                1.,
            ))
            .with_translation(Vec3::new(-WINDOW_SIZE / 2.0, -WINDOW_SIZE / 2.0, 0.0)),
        ..Default::default()
    });
}

fn collision_tile_size() -> f32 {
    TILE_SIZE * WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0) / 2.0
}

fn init_added_collision(
    mut commands: Commands,
    cells: Query<(Entity, &GridCoords), Added<IntCellCollider>>,
) {
    for (entity, coords) in cells.iter() {
        commands.spawn((
            RigidBody::Static,
            // GravityScale(0.),
            Collider::rectangle(collision_tile_size(), collision_tile_size()),
            Transform::default().with_translation(Vec3::new(
                (coords.x as f32 - TILE_MAP_SIZE / 2.0) * collision_tile_size()
                    + collision_tile_size() / 2.0,
                (coords.y as f32 - TILE_MAP_SIZE / 2.0) * collision_tile_size()
                    + collision_tile_size() / 2.0,
                0.0,
            )),
        ));

        commands.get_entity(entity).map(|mut e| e.despawn());
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
