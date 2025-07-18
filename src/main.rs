use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::system::Commands,
};

use crate::player_controller::PlayerController;

mod player_controller;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerController)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut command: Commands) {
    command.spawn(Camera2d);
}
