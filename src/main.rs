use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::system::Commands,
    math::bounding::Aabb2d,
    prelude::*,
};

use crate::physics::*;
use crate::player_controller::PlayerController;
use crate::shared::PhysicsSet;

mod physics;
mod player_controller;
mod shared;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .configure_sets(
            FixedUpdate,
            (PhysicsSet::Input, PhysicsSet::Physics).chain(),
        )
        .add_plugins(Physics)
        .add_plugins(PlayerController)
        .run();
}

fn setup(mut command: Commands) {
    command.spawn((
        Camera2d,
        Transform::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: 1. / 40.,
            ..OrthographicProjection::default_2d()
        }),
    ));

    command.spawn((
        Sprite::from_color(Color::srgb(1., 1., 1.), Vec2::ONE),
        Transform::from_translation(Vec3::new(0., -5., 1.)).with_scale(Vec3::new(20., 1., 1.)),
        Collider(Aabb2d::new(Vec2::new(0., -5.), Vec2::new(10., 0.5))),
        Static,
    ));
}
