use avian2d::{collision::Collider, dynamics::rigid_body::RigidBody, schedule::PhysicsSet};
use bevy::{math::VectorSpace, prelude::*};
use leafwing_input_manager::prelude::*;

use self::movement::LastDirection;

mod input;
mod movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<input::PlayerActionSidescroller>::default(),
            movement::CharacterControllerPlugin,
        ))
        .add_systems(Startup, spawn_player)
        .add_systems(PostUpdate, set_player_direction);
        // .add_systems(
        //     PostUpdate,
        //     follow_player
        //         .after(PhysicsSet::Sync)
        //         .before(TransformSystem::TransformPropagate),
        // );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, server: Res<AssetServer>) {
    let texture = server.load(
        "mossy_caves/BlueWizard Animations/BlueWizard/2BlueWizardIdle/Chara - BlueIdle00000.png",
    );

    commands.spawn((
        Player,
        movement::CharacterControllerBundle::new(Collider::circle(128.)),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(256., 700., 100.)),
            texture,
            ..Default::default()
        },
        InputManagerBundle::with_map(input::PlayerActionSidescroller::default_input_map()),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(512. * 16., 512.),
        Transform::from_translation(Vec3::new(256., 256., 0.)),
    ));
}

fn follow_player(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let (Some(mut camera), Some(player)) = (camera.iter_mut().next(), player.iter().next()) else {
        return;
    };

    // camera.translation = player.translation;
}

fn set_player_direction(mut player: Query<(&mut Transform, &LastDirection), With<Player>>) {
    let Ok((mut transform, last_direction)) = player.get_single_mut() else {
        return;
    };

    transform.scale.x = last_direction.0;
}
