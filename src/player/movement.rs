use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{input::PlayerAction, Player};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
        // .add_plugins(PhysicsDebugPlugin::default());
    }
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    friction: Friction,
}

impl CharacterControllerBundle {
    pub fn new() -> Self {
        let collider = Collider::circle(3.);
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.95, 5);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0., Dir2::NEG_Y)
                .with_max_time_of_impact(0.2),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            friction: Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        }
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    action: Query<&ActionState<PlayerAction>, With<Player>>,
    mut controllers: Query<(&mut LinearVelocity,), With<CharacterController>>,
) {
    let Some(action) = action.iter().next() else {
        return;
    };

    let pair = action.clamped_axis_pair(&PlayerAction::Move);
    let cleaned = if pair.length_squared() > 0.2 {
        pair.normalize_or_zero() * 100.
    } else {
        Vec2::default()
    };

    for (mut velocity,) in controllers.iter_mut() {
        velocity.x = cleaned.x;
        velocity.y = cleaned.y;
    }
}
