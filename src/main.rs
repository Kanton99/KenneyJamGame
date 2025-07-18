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

mod physics;
mod player_controller;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Physics)
        .add_plugins(PlayerController)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut command: Commands) {
    command.spawn(Camera2d);

    command.spawn((
        Sprite::from_color(Color::srgb(1., 1., 1.), Vec2::ONE),
        Transform::from_translation(Vec3::new(0., -50., 1.)).with_scale(Vec3::new(200., 10., 1.)),
        Collider(Aabb2d::new(Vec2::new(0., -50.), Vec2::new(100., 5.))),
        Static,
    ));
}
