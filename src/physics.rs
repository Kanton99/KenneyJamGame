use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    },
    math::Vec2,
    time::Time,
    transform::components::Transform,
};

pub struct Physics;

const GRAVITY: Vec2 = Vec2::new(0., -9.81);

impl Plugin for Physics {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, gravity);
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct RigidBody;

fn gravity(rigidbodies_query: Query<&mut Transform, With<RigidBody>>, time: Res<Time>) {
    for mut transform in rigidbodies_query {
        transform.translation.y += GRAVITY.y * time.delta_secs();
        transform.translation.x += GRAVITY.x * time.delta_secs();
    }
}
