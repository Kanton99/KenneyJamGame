use crate::physics;
use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    color::Color,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{Vec2, Vec3},
    prelude::{Deref, DerefMut},
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};

pub struct PlayerController;

impl Plugin for PlayerController {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_controller);
        app.add_systems(FixedUpdate, apply_velocity);
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
) {
    let mut player_velocity = player_query.into_inner();
    player_velocity.x = 0.;
    player_velocity.y = 0.;
    if input.pressed(KeyCode::KeyW) {
        player_velocity.y += 1.;
    }

    if input.pressed(KeyCode::KeyA) {
        player_velocity.x -= 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        player_velocity.x += 1.;
    }
}

fn apply_velocity(
    mut velocity_query: Single<(&Velocity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (player_velocity, mut player_transform) = velocity_query.into_inner();
    if player_velocity.length() > 0. {
        player_transform.translation.x += player_velocity.x * PLAYER_SPEED * time.delta_secs();
        player_transform.translation.y += player_velocity.y * PLAYER_SPEED * time.delta_secs();
    }
}

fn spawn_player(mut command: Commands) {
    command.spawn((
        Player,
        Velocity(Vec2::ZERO),
        Sprite::from_color(Color::srgb(1., 0., 0.), Vec2::ONE),
        Transform::from_translation(Vec3::ZERO).with_scale(Vec2::splat(40.).extend(1.)),
        physics::RigidBody,
    ));
}
