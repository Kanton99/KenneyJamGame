use avian2d::prelude::*;
use bevy::{
    app::{App, Plugin, Startup},
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
        app.add_systems(FixedUpdate, (set_can_jump, player_controller));
    }
}

const PLAYER_SPEED: f32 = 100.;
const PLAYER_JUMP: f32 = 180.;

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Player {
    can_jump: bool,
    flipped: bool,
}

fn player_controller(
    input: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&mut LinearVelocity, &mut Player, &mut Transform)>,
) {
    let (mut player_velocity, mut player, mut transform) = player_query.into_inner();
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

fn set_can_jump(
    mut events: EventReader<CollisionStarted>,
    player_query: Single<(Entity, &mut Player)>,
) {
    let (entity, mut player) = player_query.into_inner();

    for CollisionStarted(entity1, entity2) in events.read() {
        if entity1.entity() == entity || entity2.entity() == entity {
            player.can_jump = true;
        }
    }
}

fn spawn_player(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    command.spawn((
        Player {
            can_jump: false,
            flipped: false,
        },
        RigidBody::Dynamic,
        Collider::rectangle(18., 18.),
        // Sprite::from_image(asset_server.load("kenney_tiny_dungeon/Tiles/player.png")),
        Mesh2d(meshes.add(Rectangle::new(18., 18.))),
        MeshMaterial2d(materials.add(asset_server.load("kenney_tiny_dungeon/Tiles/player.png"))),
        Transform::default(),
        CollisionEventsEnabled,
        LockedAxes::ROTATION_LOCKED,
    ));
}
