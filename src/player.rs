use avian2d::schedule::PostProcessCollisions;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

mod input;
mod movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<input::PlayerAction>::default(),
            movement::CharacterControllerPlugin,
        ))
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (input::handle_actions,))
        .add_systems(PostProcessCollisions, follow_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, server: Res<AssetServer>) {
    let texture = server.load("textures/smile.png");

    commands.spawn((
        Player,
        movement::CharacterControllerBundle::new(),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                -6.5 * 16. * 3. - 1.,
                -6.5 * 16. * 3. - 1.,
                100.,
            )),
            texture,
            ..Default::default()
        },
        InputManagerBundle::with_map(input::PlayerAction::default_input_map()),
    ));
}

fn follow_player(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let (Some(mut camera), Some(player)) = (camera.iter_mut().next(), player.iter().next()) else {
        return;
    };

    camera.translation = player.translation;
    camera.scale = Vec3::new(0.15, 0.15, 1.);
}
