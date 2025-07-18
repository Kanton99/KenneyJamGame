use bevy::{
    app::{App, FixedUpdate, Plugin},
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    },
    math::{
        Vec2,
        bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    },
    prelude::*,
    time::Time,
    transform::components::Transform,
};

pub struct Physics;

const GRAVITY: Vec2 = Vec2::new(0., -9.81);

impl Plugin for Physics {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (gravity, apply_velocity, check_collisions).chain(),
        );
    }
}

#[derive(Component, Deref, DerefMut, Default)]
#[require(Transform)]
pub struct Velocity(Vec2);

#[derive(Component)]
#[require(Transform, Velocity, Collider)]
pub struct RigidBody;

#[derive(Component, Deref)]
#[require(Transform)]
pub struct Collider(pub Aabb2d);

impl Default for Collider {
    fn default() -> Self {
        Self(Aabb2d::new(Vec2::ZERO, Vec2::splat(0.5)))
    }
}

#[derive(Component)]
pub struct Static;

fn apply_velocity(
    mut vel_query: Query<(&Velocity, &mut Transform), With<RigidBody>>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in vel_query.iter_mut() {
        if velocity.length() == 0. {
            continue;
        }
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn gravity(mut rigidbodies_query: Query<&mut Velocity, With<RigidBody>>, time: Res<Time>) {
    for mut velocity in rigidbodies_query.iter_mut() {
        velocity.y += GRAVITY.y * time.delta_secs();
        velocity.x += GRAVITY.x * time.delta_secs();
    }
}

fn check_collisions(
    mut dyn_colliders_query: Query<(&Collider, &mut Transform, &mut Velocity), Without<Static>>,
    static_colliders_query: Query<(&Collider, &Transform), With<Static>>,
) {
    for (dyn_collider, mut dyn_transform, mut velocity) in dyn_colliders_query.iter_mut() {
        for (static_collider, static_transform) in static_colliders_query.iter() {
            if let Some(corrections) = check_collision(
                &dyn_transform,
                dyn_collider,
                static_transform,
                static_collider,
            ) {
                dyn_transform.translation.x += corrections.x;
                dyn_transform.translation.y += corrections.y;

                if corrections.x != 0. {
                    velocity.x = 0.;
                }

                if corrections.y != 0. {
                    velocity.y = 0.;
                }
            }
        }
    }
}

fn check_collision(
    dyn_transform: &Transform,
    dyn_collider: &Collider,
    static_transform: &Transform,
    static_collider: &Collider,
) -> Option<Vec2> {
    let dyn_world_aabb = Aabb2d::new(
        dyn_transform.translation.truncate(),
        dyn_collider.half_size(),
    );
    let static_world_aabb = Aabb2d::new(
        static_transform.translation.truncate(),
        static_collider.half_size(),
    );

    if dyn_world_aabb.intersects(&static_world_aabb) {
        let moving_pos = dyn_transform.translation.truncate();
        let static_pos = static_transform.translation.truncate();
        let moving_half_size = dyn_collider.half_size();
        let static_half_size = static_collider.half_size();

        // Calculate overlap
        let overlap_x =
            (moving_half_size.x + static_half_size.x) - (moving_pos.x - static_pos.x).abs();
        let overlap_y =
            (moving_half_size.y + static_half_size.y) - (moving_pos.y - static_pos.y).abs();

        // Find minimum correction vector
        if overlap_x < overlap_y {
            let correction_x = if moving_pos.x < static_pos.x {
                -overlap_x
            } else {
                overlap_x
            };
            Some(Vec2::new(correction_x, 0.0))
        } else {
            let correction_y = if moving_pos.y < static_pos.y {
                -overlap_y
            } else {
                overlap_y
            };
            Some(Vec2::new(0.0, correction_y))
        }
    } else {
        None
    }
}
