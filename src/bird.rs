use std::time::Duration;

use bevy::prelude::*;
use bevy_easings::{
    custom_ease_system, CustomComponentEase, EaseFunction, EasingComponent, EasingType,
    EasingsPlugin, Lerp,
};
use heron::prelude::*;

use crate::{
    game_state::{run_if_playing, GameStartedEvent, GameState, GameStateLabel},
    DiedEvent, FlapEvent, GameResetEvent, Layer,
};

const JUMP_FORCE: f32 = 300.0;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EasingsPlugin)
            .add_startup_system(spawn_bird)
            .add_system(handle_game_started.label(GameStartedStage))
            .add_system(bird_input.after(GameStartedStage).after(GameStateLabel))
            .add_system(handle_game_reset)
            .add_system(handle_died)
            .add_system(custom_ease_system::<Rotation>.with_run_criteria(run_if_playing))
            .add_system(sync_rotation)
            .add_system(handle_bird_tilt_up.with_run_criteria(run_if_playing))
            .add_system(handle_bird_tilt_down.with_run_criteria(run_if_playing));
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct GameStartedStage;

#[derive(Component)]
struct Bird;

fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/bird.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 24.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(8.0, 5.0, 0.0),
            border_radius: Some(6.0),
        })
        .insert(Velocity::default())
        .insert(CollisionLayers::all::<Layer>().with_group(Layer::Player))
        .insert(Rotation(Quat::default()))
        .insert(Bird);
}

fn bird_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut bird: Query<&mut Velocity, With<Bird>>,
    mut flap_events: EventWriter<FlapEvent>,
    game_state: Res<GameState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) && matches!(*game_state, GameState::Playing) {
        let mut velocity = bird.single_mut();

        if velocity.linear.y < 0.0 {
            velocity.linear.y = 0.0;
        }
        velocity.linear.y += JUMP_FORCE;

        flap_events.send(FlapEvent);
    }
}

fn handle_game_started(
    mut commands: Commands,
    bird: Query<Entity, With<Bird>>,
    mut game_started_events: EventReader<GameStartedEvent>,
) {
    if game_started_events.iter().next().is_some() {
        commands.entity(bird.single()).insert(RigidBody::Dynamic);
    }
}

fn handle_game_reset(
    mut game_reset_events: EventReader<GameResetEvent>,
    mut bird: Query<(&mut Transform, &mut Velocity, &mut Rotation), With<Bird>>,
) {
    if game_reset_events.iter().next().is_some() {
        let (mut transform, mut velocity, mut rotation) = bird.single_mut();
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        velocity.linear = Vec3::default();
        velocity.angular = AxisAngle::default();
        *rotation = Rotation::default();
    }
}

fn handle_died(
    mut commands: Commands,
    mut died_events: EventReader<DiedEvent>,
    mut bird: Query<Entity, With<Bird>>,
) {
    if died_events.iter().next().is_some() {
        let entity = bird.single_mut();
        commands
            .entity(entity)
            .remove::<RigidBody>()
            .remove::<EasingComponent<Rotation>>();
    }
}

#[derive(Component, Clone, Copy, Default)]
struct Rotation(Quat);

impl Lerp for Rotation {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        Rotation(self.0.lerp(other.0, *scalar))
    }
}

fn sync_rotation(mut query: Query<(&mut Transform, &Rotation)>) {
    for (mut transform, rotation) in query.iter_mut() {
        transform.rotation = rotation.0;
    }
}

#[derive(Component)]
struct TiltUp;

#[derive(Component)]
struct TiltDown;

fn handle_bird_tilt_up(
    mut commands: Commands,
    bird: Query<(Entity, &Velocity, &Rotation), (With<Bird>, Without<TiltUp>)>,
) {
    if let Ok((entity, velocity, rotation)) = bird.get_single() {
        if velocity.linear.y > 0.0 {
            let ease = rotation.ease_to(
                Rotation(Quat::from_rotation_z(0.3)),
                EaseFunction::QuadraticOut,
                EasingType::Once {
                    duration: Duration::from_millis(400),
                },
            );

            let mut entity = commands.entity(entity);
            entity
                .remove::<EasingComponent<Rotation>>()
                .remove::<TiltDown>()
                .insert(ease)
                .insert(TiltUp);
        }
    }
}

fn handle_bird_tilt_down(
    mut commands: Commands,
    bird: Query<(Entity, &Velocity, &Rotation), (With<Bird>, Without<TiltDown>)>,
) {
    if let Ok((entity, velocity, rotation)) = bird.get_single() {
        if velocity.linear.y < 0.0 {
            let ease = rotation.ease_to(
                Rotation(Quat::from_rotation_z(-0.3)),
                EaseFunction::QuadraticOut,
                EasingType::Once {
                    duration: Duration::from_millis(800),
                },
            );

            let mut entity = commands.entity(entity);
            entity
                .remove::<EasingComponent<Rotation>>()
                .remove::<TiltUp>()
                .insert(ease)
                .insert(TiltDown);
        }
    }
}
