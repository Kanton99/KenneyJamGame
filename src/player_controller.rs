use avian2d::prelude::*;
use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        system::{Commands, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec2,
    prelude::*,
    sprite::Sprite,
    transform::components::Transform,
};
use bevy_ecs_ldtk::prelude::*;

use crate::GameLayer;

pub struct PlayerController;

impl Plugin for PlayerController {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_ground_sensor);
        app.add_systems(Update, (player_controller, move_sensor).chain());
        app.register_ldtk_entity::<PlayerBundle>("Player");
    }
}

const PLAYER_SPEED: f32 = 100.;
const PLAYER_JUMP: f32 = 80.;

#[derive(Component, Default)]
#[require(
    RigidBody::Dynamic,
    Collider::rectangle(18., 18.),
    LockedAxes::ROTATION_LOCKED,
    Friction::new(0.2),
    Restitution::new(0.),
    CollisionLayers::new(GameLayer::Player, [GameLayer::Default, GameLayer::Ground])
)]
pub struct Player {
    flipped: bool,
    with_child: bool,
    jumped: bool,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
    player: Player,
}

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

fn player_controller(
    input: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&mut LinearVelocity, &mut Player, &mut Transform)>,
    sensor_query: Single<&CollidingEntities, With<GroundSensor>>,
) {
    let (mut player_velocity, mut player, mut transform) = player_query.into_inner();
    let sensor = sensor_query.into_inner();
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::Space) && !sensor.is_empty() {
        direction.y = PLAYER_JUMP;
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x = -PLAYER_SPEED;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x = PLAYER_SPEED;
    }

    if player.flipped {
        if direction.x > 0. {
            player.flipped = false;
        }
    } else if direction.x < 0. {
        player.flipped = true;
    }

    transform.scale.x = if player.flipped { -1. } else { 1. };

    player_velocity.x = direction.x;
    player_velocity.y += direction.y;
}

#[derive(Component)]
pub struct GroundSensor;

fn add_ground_sensor(mut commands: Commands, player_query: Single<&mut Player>) {
    let mut player = player_query.into_inner();
    if player.with_child {
        return;
    }

    commands.spawn((
        RigidBody::Kinematic,
        Collider::rectangle(10., 1.),
        Sensor,
        GroundSensor,
        CollidingEntities::default(),
        Transform::from_xyz(0., -10., 0.),
        CollisionLayers::new(GameLayer::GroundSensor, [GameLayer::Ground]),
    ));

    // commands.entity(entity).add_child(sensor_id);
    player.with_child = true;
}

fn move_sensor(
    player_query: Single<(&Transform, &LinearVelocity), (With<Player>, Without<GroundSensor>)>,
    sensor_query: Single<&mut Transform, (With<GroundSensor>, Without<Player>)>,
) {
    let (player, velocity) = player_query.into_inner();
    let mut sensor = sensor_query.into_inner();

    let offset: Vec2 = Vec2::new(0., -12.);

    sensor.translation.x = player.translation.x + offset.x;
    sensor.translation.y = player.translation.y + offset.y;
}
