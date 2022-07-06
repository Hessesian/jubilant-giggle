use std::f32::consts::PI;

use bevy::prelude::*;
use heron::prelude::*;
use crate::actions::Actions;
use crate::GameState;
use crate::loading::TextureAssets;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera)
                .with_system(setup_bg),
        )
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player))
            .add_startup_system(setup_graphics);
    }
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_bg(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bg.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)).with_scale(Vec3::new(0.9, 0.9, 0.1)),
            ..Default::default()
        });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    /* Create the bouncing ball. */
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 200.0, 1.0).with_scale(Vec3::new(0.1, 0.1, 1.)),
            texture: textures.texture_ball.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        // Attach a collision shape
        .insert(CollisionShape::Sphere {
            radius: 15.0
        })
        // Add an initial velocity. (it is also possible to read/mutate this component later)
        .insert(Velocity::from(Vec2::X * 300.0).with_angular(AxisAngle::new(Vec3::Z, -PI)))
        // Define restitution (so that it bounces)
        .insert(PhysicMaterial {
            restitution: 1.5,
            ..Default::default()
        });

    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)).with_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..Default::default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from_linear(Vec3::X * 2.0))
        .insert(Acceleration::from_linear(Vec3::X * 1.0))
        .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Sphere { radius: 10.0 });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 30.;
    let movement = Vec2::new(
        actions.player_movement.unwrap().x,
        actions.player_movement.unwrap().y).normalize_or_zero().extend(0.0) * speed;


    for mut player_transform in player_query.iter_mut() {
        player_transform.linear += movement;
    }
}
