use avian2d::prelude::*;
use avian2d::spatial_query::SpatialQuery;
use avian2d::spatial_query::SpatialQueryFilter;
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
        app.add_systems(Update, update_spawn_point);
        app.add_systems(Update, (ground_check, player_controller, win).chain());
        app.register_ldtk_entity::<PlayerBundle>("Player");
        app.insert_resource(SpawnPoint { pos: Vec2::ZERO });
    }
}

const PLAYER_SPEED: f32 = 100.;
const PLAYER_JUMP: f32 = 320.;

#[derive(Component, Default)]
#[require(
    RigidBody::Dynamic,
    Collider::rectangle(18., 18.),
    LockedAxes::ROTATION_LOCKED,
    Friction::new(0.2),
    Restitution::new(0.),
    CollisionLayers::new(GameLayer::Player, [GameLayer::Default, GameLayer::Ground]),
    CollisionEventsEnabled
)]
pub struct Player {
    flipped: bool,
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
) {
    let (mut player_velocity, mut player, mut transform) = player_query.into_inner();
    let mut direction = Vec2::ZERO;
    if !player.jumped && input.pressed(KeyCode::Space) {
        direction.y = PLAYER_JUMP;
        player.jumped = true;
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

#[derive(Resource)]
pub struct SpawnPoint {
    pub pos: Vec2,
}

fn update_spawn_point(
    player_query: Single<&mut Transform, Added<Player>>,
    mut spawn_point: ResMut<SpawnPoint>,
) {
    let mut transform = player_query.into_inner();
    transform.translation.z = 10.;
    spawn_point.pos = transform.translation.truncate();
}

fn win(mut commands: Commands, player_query: Single<&Transform, With<Player>>) {
    let player = player_query.into_inner();
    if player.translation.x >= 913. {
        let mut translation = player.translation;
        translation.y += 50.;
        commands.spawn((
            Text::new("You found More Power"),
            Transform::from_translation(translation),
        ));
    }
}

fn ground_check(player_pos: Single<(&mut Player, &Transform)>, spatial_query: SpatialQuery) {
    let (mut player, transform) = player_pos.into_inner();

    let origin = transform.translation.truncate();
    let direction = Dir2::NEG_Y;
    let filter = SpatialQueryFilter::from_mask([GameLayer::Ground]);

    if let Some(_hit) = spatial_query.cast_ray(origin, direction, 9.1, true, &filter) {
        player.jumped = false;
    }
}
