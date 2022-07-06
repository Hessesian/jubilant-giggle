use bevy::prelude::*;
use heron::prelude::*;
use heron::rapier_plugin::RapierPlugin;

pub struct GamePhysicsPlugin;

impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_physics)
            .add_plugin(RapierPlugin::default())
            //.add_plugin(DebugPlugin::default())
            .insert_resource(Gravity::from(Vec2::new(0.0, -600.0)));
    }
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    let size = Vec2::new(1000.0, 50.0);
    let wall_size = Vec2::new(50.0, 1000.0);
    commands
        .spawn()
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(RigidBody::Static)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(50.0, -200.0, 0.0)));

    commands
        .spawn()
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(RigidBody::Static)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(50.0, 300.0, 0.0)));

    commands
        .spawn()
        .insert(CollisionShape::Cuboid {
            half_extends: wall_size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(RigidBody::Static)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(-250.0, -250.0, 0.0)));

    commands
        .spawn()
        .insert(CollisionShape::Cuboid {
            half_extends: wall_size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(RigidBody::Static)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(250.0, -250.0, 0.0)));
}