use avian2d::{collision::Collider, dynamics::rigid_body::RigidBody};
use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings};
use bevy::prelude::*;
use bevy::render::view::{ColorGrading, ColorGradingGlobal, RenderLayers};
use bevy_ecs_ldtk::prelude::*;
use bevy_hanabi::prelude::*;

pub const WINDOW_SIZE: f32 = 1000.0;
const TILE_SIZE: f32 = 512.0;
const TILE_MAP_SIZE: f32 = 16.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LdtkPlugin, HanabiPlugin))
            .register_ldtk_entity::<GoalBundle>("Goal")
            .register_ldtk_int_cell_for_layer::<ColliderBundle>("Collision", 1)
            .insert_resource(LevelSelection::index(0))
            .add_systems(Startup, (setup, setup_effect))
            .add_systems(Update, init_added_collision);
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut settings: ResMut<LdtkSettings>,
    mut clear_color: ResMut<ClearColor>,
) {
    *clear_color = ClearColor(Color::LinearRgba(LinearRgba::new(
        0.000, 1.000, 0.270, 0.728,
    )));

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                order: 0,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            ..Default::default()
        },
        BloomSettings {
            intensity: 0.2,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.4,
                threshold_softness: 0.2,
            },
            composite_mode: BloomCompositeMode::Additive,
        },
        RenderLayers::layer(0),
    ));
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("map.ldtk"),
            transform: Transform::default()
                .with_scale(Vec3::new(
                    WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0),
                    WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0),
                    1.,
                ))
                .with_translation(Vec3::new(-WINDOW_SIZE / 2.0, -WINDOW_SIZE / 2.0, 0.0)),
            ..Default::default()
        },
        RenderLayers::layer(0),
    ));
    settings.set_clear_color = SetClearColor::No;
    settings.level_background = LevelBackground::Nonexistent;

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                order: -1,
                ..Default::default()
            },
            ..Default::default()
        },
        ColorGrading {
            global: ColorGradingGlobal {
                exposure: 1.0,
                // temperature: (),
                tint: 100.0,
                // hue: (),
                post_saturation: 1.0,
                ..Default::default()
            },
            ..Default::default()
        },
        BloomSettings {
            intensity: 0.8,
            low_frequency_boost: 0.0,
            low_frequency_boost_curvature: 0.0,
            high_pass_frequency: 1.0 / 3.0,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.0,
                threshold_softness: 0.0,
            },
            composite_mode: BloomCompositeMode::EnergyConserving,
        },
        // FogSettings {
        //     color: Color::srgb(0.25, 0.25, 0.25),
        //     falloff: FogFalloff::Linear {
        //         start: 5.0,
        //         end: 20.0,
        //     },
        //     ..default()
        // },
        RenderLayers::layer(1),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::default().with_scale(Vec3::new(
                WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0),
                WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0),
                1.,
            )),
            //.with_translation(Vec3::new(-WINDOW_SIZE / 2.0, -WINDOW_SIZE / 2.0, 0.0)),
            texture: asset_server.load("background/simplified/Level_0/Walls2.png"),
            sprite: Sprite {
                // color: Color::LinearRgba(LinearRgba::new(0.0, 0.0, 0.0, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::default().with_scale(Vec3::new(
                WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0),
                WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0),
                1.,
            )),
            //.with_translation(Vec3::new(-WINDOW_SIZE / 2.0, -WINDOW_SIZE / 2.0, 0.0)),
            texture: asset_server.load("background/simplified/Level_0/Background_decor2.png"),
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::default().with_scale(Vec3::new(
                WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0),
                WINDOW_SIZE / (TILE_SIZE * TILE_MAP_SIZE / 2.0),
                1.,
            )),
            //.with_translation(Vec3::new(-WINDOW_SIZE / 2.0, -WINDOW_SIZE / 2.0, 0.0)),
            texture: asset_server.load("background/simplified/Level_0/Plants2.png"),
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));
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

fn setup_effect(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0., 0.8, 0.2, 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(500.),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::new(-20., -20., 0.)),
        speed: module.lit(20.),
    };

    let init_size = SetSizeModifier {
        size: CpuValue::Uniform((Vec2::splat(2.), Vec2::splat(8.))),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(10.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., 0., 0.));
    let update_accel = AccelModifier::new(accel);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        vec![32768],
        // Spawn at a rate of 5 particles per second
        Spawner::rate(5.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("MyEffect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .update(update_accel)
    .render(init_size)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_asset = effects.add(effect);

    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_asset.clone()),
            transform: Transform::from_translation(Vec3::Y).with_scale(Vec3::splat(5.)),
            ..Default::default()
        },
        RenderLayers::layer(0),
    ));

    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_asset),
            transform: Transform::from_translation(Vec3::Y).with_scale(Vec3::splat(5.)),
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));
}
