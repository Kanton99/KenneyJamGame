use crate::physics::*;
use bevy::{
    app::{App, Plugin, Startup, Update},
    color::Color,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{Vec2, Vec3, bounding::Aabb2d},
    prelude::*,
    sprite::Sprite,
    transform::components::Transform,
};

pub struct PlayerController;

impl Plugin for PlayerController {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_controller);
    }
}

const PLAYER_SPEED: f32 = 100.;

#[derive(Component)]
#[require(Sprite, Transform)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn player_controller(
    input: Res<ButtonInput<KeyCode>>,
    player_query: Single<&mut Velocity, With<Player>>,
    time: Res<Time>,
) {
    let mut player_velocity = player_query.into_inner();
    let mut direction = Vec2::ZERO;
    // player_velocity.x = 0.;
    // player_velocity.y = 0.;
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1. * PLAYER_SPEED * time.delta_secs();
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1. * PLAYER_SPEED * time.delta_secs();
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1. * PLAYER_SPEED * time.delta_secs();
    }

    player_velocity.x += direction.x;
    player_velocity.y += direction.y;
}

fn spawn_player(mut command: Commands) {
    command.spawn((
        Player,
        Collider(Aabb2d::new(Vec2::ZERO, Vec2::splat(20.))),
        Sprite::from_color(Color::srgb(1., 0., 0.), Vec2::ONE),
        Transform::from_translation(Vec3::ZERO).with_scale(Vec2::splat(40.).extend(1.)),
        RigidBody,
        Velocity(Vec2::ZERO),
    ));
}
