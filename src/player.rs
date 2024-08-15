use self::movement::LastDirection;
use bevy::prelude::*;
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkEntity, LdtkSpriteSheetBundle};
use input::PlayerActionSidescroller;
use leafwing_input_manager::prelude::*;
use movement::{CharacterControllerBundle, Grounded};

pub mod input;
pub mod movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_plugins((
                InputManagerPlugin::<input::PlayerActionSidescroller>::default(),
                movement::CharacterControllerPlugin,
            ))
            .add_systems(PostUpdate, (set_player_direction, animate_sprite));
        // .add_systems(
        //     PostUpdate,
        //     follow_player
        //         .after(PhysicsSet::Sync)
        //         .before(TransformSystem::TransformPropagate),
        // );
    }
}

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    player: Player,
    state: PlayerState,
    movement: CharacterControllerBundle,
    input: InputManagerBundle<PlayerActionSidescroller>,
    juice: JuiceMeter,
    animation_timer: AnimationTimer,
    animation_indices: AnimationIndices,
}

#[derive(Component, Debug, Default, PartialEq, Eq)]
pub enum PlayerState {
    #[default]
    Idle,
    Walk,
    Jump,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

impl AnimationIndices {
    const IDLE: Self = Self { first: 0, last: 19 };
    const WALK: Self = Self {
        first: 20,
        last: 39,
    };
    const JUMP: Self = Self {
        first: 40,
        last: 47,
    };
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            state: PlayerState::Idle,
            sprite_sheet_bundle: Default::default(),
            input: InputManagerBundle::with_map(PlayerActionSidescroller::default_input_map()),
            movement: CharacterControllerBundle::default(),
            juice: Default::default(),
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            animation_indices: AnimationIndices { first: 0, last: 19 },
        }
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub enum JuiceMeter {
    #[default]
    Idle,
    Charging(f32),
    Exhausted,
}

fn follow_player(
    mut cameras: Query<&mut Transform, With<Camera2d>>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    for mut transform in cameras.iter_mut() {
        transform.translation = player.translation;
    }
}

fn set_player_direction(mut player: Query<(&mut Transform, &LastDirection), With<Player>>) {
    let Ok((mut transform, last_direction)) = player.get_single_mut() else {
        return;
    };

    transform.scale.x = transform.scale.x.abs() * last_direction.0;
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &mut PlayerState,
        Option<&Grounded>,
        Option<&ActionState<PlayerActionSidescroller>>,
    )>,
) {
    for (mut indices, mut timer, mut atlas, mut state, grounded, action) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if grounded.is_some() {
                if let Some(action) = action {
                    let val = action
                        .axis_data(&PlayerActionSidescroller::Move)
                        .unwrap()
                        .value;
                    const DELTA: f32 = 0.1;

                    if (val >= DELTA || val <= -DELTA)
                        && !action.pressed(&PlayerActionSidescroller::Jump)
                    {
                        if *state != PlayerState::Walk {
                            *state = PlayerState::Walk;
                            atlas.index = AnimationIndices::WALK.first;
                        }
                    } else {
                        if *state != PlayerState::Idle {
                            *state = PlayerState::Idle;
                            atlas.index = AnimationIndices::IDLE.first;
                        }
                    }
                }
            } else {
                if *state != PlayerState::Jump {
                    *state = PlayerState::Jump;
                    atlas.index = AnimationIndices::JUMP.first;
                }
            }

            match *state {
                PlayerState::Idle => *indices = AnimationIndices::IDLE,
                PlayerState::Walk => *indices = AnimationIndices::WALK,
                PlayerState::Jump => *indices = AnimationIndices::JUMP,
            }

            atlas.index = if atlas.index >= indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
