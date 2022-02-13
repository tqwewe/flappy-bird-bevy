use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;

use crate::{GameResetEvent, GameState, Layer, WIN_HEIGHT, WIN_WIDTH};

const PIPE_SPACING_X: f32 = (WIN_WIDTH / 5.0) + (PIPE_WIDTH / 5.0);
const PIPE_SPACING_Y: f32 = 130.0; // 150.0;
const PIPE_WIDTH: f32 = 80.0;
const PIPE_HEIGHT: f32 = WIN_HEIGHT - PIPE_WIDTH / 2.0;
const PIPE_PADDING: f32 = 200.0;
const PIPE_SPEED: f32 = 100.0;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_pipes)
            .add_system(move_pipes)
            .add_system(handle_game_reset);
    }
}

#[derive(Component)]
struct Pipe;

#[derive(Component)]
struct PipePair;

fn spawn_pipes(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..5 {
        if i == 0 {
            spawn_pipe(&mut commands, &asset_server, 0.0, 0.0);
        } else {
            let y = rand_y_pos();
            spawn_pipe(&mut commands, &asset_server, i as f32 * PIPE_SPACING_X, y);
        }
    }
}

fn rand_y_pos() -> f32 {
    rand::thread_rng().gen_range(-1.0..1.0)
}

fn spawn_pipe(commands: &mut Commands, asset_server: &AssetServer, mut x: f32, y: f32) {
    let offset = (WIN_HEIGHT - PIPE_PADDING * 2.0) * (y / 2.0);
    x += 300.0;

    commands
        .spawn()
        .insert(Transform::from_xyz(x, offset, 0.0))
        .insert(GlobalTransform::default())
        .insert(PipePair)
        .with_children(|commands| {
            // Top pipe
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("sprites/pipe.png"),
                    sprite: Sprite {
                        // color: Color::rgb(0.75, 0.0, 0.25),
                        custom_size: Some(Vec2::new(PIPE_WIDTH, PIPE_HEIGHT)),
                        flip_y: true,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        0.0,
                        PIPE_HEIGHT / 2.0 + PIPE_SPACING_Y / 2.0,
                        0.0,
                    ),
                    ..Default::default()
                })
                .insert(RigidBody::KinematicPositionBased)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(PIPE_WIDTH / 2.0, PIPE_HEIGHT / 2.0, 0.0),
                    border_radius: None,
                })
                // .insert(PhysicMaterial {
                //     friction: 1.0,
                //     density: 10.0,
                //     restitution: 3.0,
                // })
                .insert(
                    CollisionLayers::none()
                        .with_group(Layer::World)
                        .with_mask(Layer::Player),
                )
                .insert(Pipe);

            // Bottom pipe
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("sprites/pipe.png"),
                    sprite: Sprite {
                        // color: Color::rgb(0.25, 0.0, 0.75),
                        custom_size: Some(Vec2::new(PIPE_WIDTH, PIPE_HEIGHT)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        0.0,
                        ((PIPE_HEIGHT / 2.0) * -1.0) - PIPE_SPACING_Y / 2.0,
                        0.0,
                    ),
                    ..Default::default()
                })
                .insert(RigidBody::KinematicPositionBased)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(PIPE_WIDTH / 2.0, PIPE_HEIGHT / 2.0, 0.0),
                    border_radius: None,
                })
                // .insert(PhysicMaterial {
                //     friction: 1.0,
                //     density: 10.0,
                //     restitution: 3.0,
                // })
                .insert(
                    CollisionLayers::none()
                        .with_group(Layer::World)
                        .with_mask(Layer::Player),
                )
                .insert(Pipe);

            // Pipe gap sensor
            commands
                .spawn_bundle((Transform::default(), GlobalTransform::default()))
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(
                        PIPE_WIDTH / 2.0 - 0.5,
                        PIPE_SPACING_Y / 2.0 - 0.5,
                        0.0,
                    ),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(Layer::PipeGap)
                        .with_mask(Layer::Player),
                )
                .insert(Pipe);
        });
}

fn move_pipes(
    time: Res<Time>,
    mut pipes: Query<&mut Transform, With<PipePair>>,
    game_state: Res<GameState>,
) {
    if matches!(*game_state, GameState::Playing) {
        for mut pipe in pipes.iter_mut() {
            pipe.translation.x -= PIPE_SPEED * time.delta_seconds();

            if pipe.translation.x < (WIN_WIDTH / 2.0 + PIPE_WIDTH / 2.0) * -1.0 {
                pipe.translation.x = WIN_WIDTH / 2.0 + PIPE_WIDTH / 2.0;

                let pos = rand_y_pos();
                let offset = (WIN_HEIGHT - PIPE_PADDING * 2.0) * (pos / 2.0);
                pipe.translation.y = offset;
            }
        }
    }
}

fn handle_game_reset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_reset_events: EventReader<GameResetEvent>,
    mut pipes: QuerySet<(
        QueryState<Entity, With<Pipe>>,
        QueryState<Entity, With<PipePair>>,
    )>,
) {
    if game_reset_events.iter().next().is_some() {
        let q0 = pipes.q0();
        for entity in q0.iter() {
            commands.entity(entity).despawn();
        }
        let q1 = pipes.q1();
        for entity in q1.iter() {
            commands.entity(entity).despawn();
        }
        spawn_pipes(commands, asset_server);
    }
}
