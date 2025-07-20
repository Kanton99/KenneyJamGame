use std::collections::HashMap;

use crate::player_controller::*;
use avian2d::prelude::*;
use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::system::Commands,
    prelude::*,
};
use bevy_ecs_ldtk::prelude::*;

mod player_controller;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(9.),
            PhysicsDebugPlugin::default(),
            LdtkPlugin,
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 320.0))
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, setup)
        .add_plugins(PlayerController)
        .add_systems(Update, (camera_follow, spawn_wall_colliders))
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(1280. / 4., 720. / 4., 0.),
        Projection::Orthographic(OrthographicProjection {
            scale: 2. / 4.5,
            ..OrthographicProjection::default_2d()
        }),
        ElasticCamera::default(),
    ));

    // command.spawn((
    //     Sprite::from_color(Color::srgb(1., 1., 1.), Vec2::ONE),
    //     Transform::from_translation(Vec3::new(0., -50., 1.)).with_scale(Vec3::new(200., 1., 1.)),
    //     RigidBody::Static,
    //     Collider::rectangle(1., 1.),
    // ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server
            .load("ldtk_project/the_search_for_more_power.ldtk")
            .into(),
        ..Default::default()
    });
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

fn spawn_wall_colliders(
    mut commands: Commands,
    wall_query: Query<(Entity, &Transform), Added<IntGridCell>>,
) {
    let mut grid: HashMap<(i32, i32), Entity> = HashMap::new();
    let mut processed = std::collections::HashSet::new();

    // Collect all wall positions
    for (entity, transform) in wall_query.iter() {
        let grid_x = (transform.translation.x / 16.0).round() as i32;
        let grid_y = (transform.translation.y / 16.0).round() as i32;
        grid.insert((grid_x, grid_y), entity);
    }

    // Process each wall tile
    for (entity, transform) in wall_query.iter() {
        if processed.contains(&entity) {
            continue;
        }

        let grid_x = (transform.translation.x / 16.0).round() as i32;
        let grid_y = (transform.translation.y / 16.0).round() as i32;

        // Find horizontal span
        let mut width = 1;
        while grid.contains_key(&(grid_x + width, grid_y)) {
            if let Some(&next_entity) = grid.get(&(grid_x + width, grid_y)) {
                processed.insert(next_entity);
                // commands.entity(next_entity).despawn();
            }
            width += 1;
        }

        // Create merged collider
        commands.entity(entity).insert((
            RigidBody::Static,
            Collider::rectangle(16.0 * width as f32, 16.0),
            CollisionLayers::new(
                GameLayer::Ground,
                [
                    GameLayer::Default,
                    GameLayer::Player,
                    GameLayer::GroundSensor,
                ],
            ),
        ));

        // Adjust position to center of merged collider
        let center_x = transform.translation.x + (width as f32 - 1.0) * 8.0;
        commands.entity(entity).insert(Transform::from_xyz(
            center_x,
            transform.translation.y,
            transform.translation.z,
        ));

        processed.insert(entity);
    }
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Ground,
    GroundSensor,
}
