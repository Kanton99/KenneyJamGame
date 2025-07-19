use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::system::Commands,
    math::bounding::Aabb2d,
    prelude::*,
};

use crate::physics::*;
use crate::player_controller::*;
use crate::shared::PhysicsSet;

mod physics;
mod player_controller;
mod shared;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .configure_sets(
            FixedUpdate,
            (PhysicsSet::Input, PhysicsSet::Physics).chain(),
        )
        .add_plugins(Physics)
        .add_plugins(PlayerController)
        .add_systems(Update, camera_follow)
        .run();
}

#[derive(Component)]
struct ElasticCamera {
    lag_distance: f32,
    catch_up_speed: f32,
}

impl Default for ElasticCamera {
    fn default() -> Self {
        Self {
            lag_distance: 0.1,
            catch_up_speed: 2.,
        }
    }
}

fn setup(mut command: Commands) {
    command.spawn((
        Camera2d,
        Transform::from_xyz(0., 0., 0.),
        Projection::Orthographic(OrthographicProjection {
            scale: 1. / (18. * 4.),
            ..OrthographicProjection::default_2d()
        }),
        ElasticCamera::default(),
    ));

    command.spawn((
        Sprite::from_color(Color::srgb(1., 1., 1.), Vec2::ONE),
        Transform::from_translation(Vec3::new(0., -5., 1.)).with_scale(Vec3::new(20., 1., 1.)),
        Collider(Aabb2d::new(Vec2::new(0., -5.), Vec2::new(10., 0.5))),
        Static,
    ));
}

fn camera_follow(
    player_query: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    camera_query: Single<(&mut Transform, &ElasticCamera), (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    let player = player_query.into_inner();
    let (mut camera, elastic_params) = camera_query.into_inner();

    let player_pos = player.translation.truncate();
    let camera_pos = camera.translation.truncate();

    let distance = player_pos.distance(camera_pos);
    // Only move camera if player is beyond lag distance
    if distance > elastic_params.lag_distance {
        let direction = (player_pos - camera_pos).normalize();
        let target_pos = player_pos - direction * elastic_params.lag_distance;

        // Smooth movement toward target
        let new_pos = camera_pos.lerp(
            target_pos,
            elastic_params.catch_up_speed * time.delta_secs(),
        );

        camera.translation.x = new_pos.x;
        camera.translation.y = new_pos.y;
    }
}
