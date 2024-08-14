use super::{input::PlayerActionSidescroller, JuiceMeter, Player};
use crate::GRAVITY;
use avian2d::{math::*, prelude::*};
use bevy::{ecs::query::Has, prelude::*};
use leafwing_input_manager::action_state::ActionState;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPreUpdate,
            (update_grounded, movement, handle_jump, handle_elasticity)
                .chain()
                .before(PhysicsSet::Prepare)
                .before(PhysicsSet::StepSimulation),
        );
    }
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementSpeed(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The last direction the player faced when moving.
#[derive(Component)]
pub struct LastDirection(pub Scalar);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
    restitution: Restitution,
    friction: Friction,
    margin: CollisionMargin,
}

impl Default for CharacterControllerBundle {
    fn default() -> Self {
        Self::new(Collider::rectangle(128., 256.))
    }
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    speed: MovementSpeed,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
    last_direction: LastDirection,
}

impl MovementBundle {
    pub const fn new(acceleration: Scalar, jump_impulse: Scalar, max_slope_angle: Scalar) -> Self {
        Self {
            speed: MovementSpeed(acceleration),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
            last_direction: LastDirection(1.),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(GRAVITY * 0.2, GRAVITY * 0.65, PI * 0.45)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.48, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_time_of_impact(10.),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
            restitution: Restitution::PERFECTLY_INELASTIC
                .with_combine_rule(CoefficientCombine::Min),
            friction: Friction::ZERO.with_combine_rule(CoefficientCombine::Min),

            margin: CollisionMargin(2.),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, jump_impulse, max_slope_angle);
        self
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    action: Query<&ActionState<PlayerActionSidescroller>, With<Player>>,
    mut controllers: Query<(
        &MovementSpeed,
        &JuiceMeter,
        &mut LinearVelocity,
        &mut LastDirection,
        &mut Transform,
        Has<Grounded>,
    )>,
) {
    let Ok(action) = action.get_single() else {
        return;
    };

    for (speed, juice, mut linear_velocity, mut last_direction, mut transform, is_grounded) in
        &mut controllers
    {
        let value = action.axis_data(&PlayerActionSidescroller::Move).unwrap();

        // jump king controls

        if is_grounded {
            if matches!(juice, JuiceMeter::Charging(_)) {
                linear_velocity.x = 0.;
            } else if value.value > 0.2 {
                last_direction.0 = 1.;
                linear_velocity.x = speed.0;
            } else if value.value < -0.2 {
                last_direction.0 = -1.;
                linear_velocity.x = -speed.0;
            } else {
                linear_velocity.x = 0.;
            }

            linear_velocity.y = linear_velocity.y.max(0.);
        }
    }
}

fn handle_jump(
    action: Query<&ActionState<PlayerActionSidescroller>, With<Player>>,
    mut controllers: Query<(
        &JumpImpulse,
        &mut LinearVelocity,
        &mut LastDirection,
        &mut Transform,
        &mut JuiceMeter,
        Has<Grounded>,
    )>,
    time: Res<Time>,
) {
    let Ok(action) = action.get_single() else {
        return;
    };

    for (
        jump_impulse,
        mut linear_velocity,
        last_direction,
        mut transform,
        mut juice,
        is_grounded,
    ) in &mut controllers
    {
        match *juice {
            JuiceMeter::Idle => {
                if is_grounded && action.just_pressed(&PlayerActionSidescroller::Jump) {
                    *juice = JuiceMeter::Charging(0.);
                }
            }
            JuiceMeter::Charging(charge) => {
                let charge = charge + time.delta_seconds();

                let released = action.just_released(&PlayerActionSidescroller::Jump);
                let overcharged = charge >= 1.;

                if released {
                    *juice = JuiceMeter::Idle;
                } else if overcharged {
                    *juice = JuiceMeter::Exhausted;
                } else {
                    *juice = JuiceMeter::Charging(charge);
                }

                if charge >= 0.15 && (released || overcharged) {
                    if is_grounded {
                        linear_velocity.x =
                            jump_impulse.0 * last_direction.0 * 0.5 * (0.1 + charge);
                        linear_velocity.y = jump_impulse.0 * (0.1 + charge);
                        transform.translation.y += 20.;
                    }
                }
            }
            JuiceMeter::Exhausted => {
                if action.just_released(&PlayerActionSidescroller::Jump) {
                    *juice = JuiceMeter::Idle;
                }
            }
        };
    }
}

fn handle_elasticity(mut controllers: Query<(&LinearVelocity, &mut Restitution, Has<Grounded>)>) {
    for (velocity, mut rest, is_grounded) in controllers.iter_mut() {
        if !is_grounded && velocity.y > 100. {
            *rest = Restitution::PERFECTLY_ELASTIC;
        } else {
            *rest = Restitution::PERFECTLY_INELASTIC;
        }
    }
}
