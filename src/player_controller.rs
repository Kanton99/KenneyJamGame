use crate::physics::*;
use crate::shared::PhysicsSet;
use bevy::{
    app::{App, Plugin, Startup},
    color::Color,
    ecs::{
        component::Component,
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
        app.add_systems(
            FixedUpdate,
            (set_can_jump, player_controller.in_set(PhysicsSet::Input)),
        );
    }
}

const PLAYER_SPEED: f32 = 5.;
const PLAYER_JUMP: f32 = 3.;

#[derive(Component)]
#[require(Sprite, Transform)]
struct Player {
    can_jump: bool,
}

fn player_controller(
    input: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&mut Velocity, &mut Player)>,
) {
    let (mut player_velocity, mut player) = player_query.into_inner();
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::Space) && player.can_jump {
        direction.y = PLAYER_JUMP;
        player.can_jump = false;
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x = -PLAYER_SPEED;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x = PLAYER_SPEED;
    }

    player_velocity.x = direction.x;
    player_velocity.y += direction.y;
}

fn set_can_jump(
    mut events: EventReader<CollisionEvent>,
    player_query: Single<(Entity, &mut Player)>,
) {
    let (entity, mut player) = player_query.into_inner();

    for event in events.read() {
        if event.this == entity {
            player.can_jump = true;
        }
    }
}

fn spawn_player(mut command: Commands) {
    command.spawn((
        Player { can_jump: false },
        Collider(Aabb2d::new(Vec2::ZERO, Vec2::splat(0.5))),
        Sprite::from_color(Color::srgb(1., 0., 0.), Vec2::ONE),
        Transform::from_translation(Vec3::ZERO).with_scale(Vec2::splat(1.).extend(1.)),
        RigidBody,
    ));
}
