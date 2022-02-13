use std::time::Duration;

use bevy::{prelude::*, render::mesh::shape, sprite::MaterialMesh2dBundle};
use bevy_easings::{
    custom_ease_system, CustomComponentEase, Ease, EaseFunction, EasingComponent, EasingType,
    EasingsPlugin, Lerp,
};
use heron::prelude::*;

use crate::{world::WorldStage, FlapEvent, GameResetEvent, GameState, Layer};

const JUMP_FORCE: f32 = 300.0;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EasingsPlugin)
            .add_startup_system(spawn_bird)
            .add_system_set(
                SystemSet::new()
                    .label(BirdStage)
                    .with_system(bird_input)
                    .with_system(handle_game_reset),
            )
            .add_system(handle_game_reset)
            .add_system(custom_ease_system::<Rotation>)
            .add_system(sync_rotation)
            .add_system(handle_bird_tilt_up)
            .add_system(handle_bird_tilt_down);
        // .add_system(handle_bird_tilt);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub struct BirdStage;

#[derive(Component)]
struct Bird;

fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/bird.png"),
            sprite: Sprite {
                // color: Color::rgb(0.25, 0.0, 0.75),
                custom_size: Some(Vec2::new(32.0, 24.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        // .spawn_bundle(MaterialMesh2dBundle {
        //     mesh: meshes
        //         .add(Mesh::from(shape::Icosphere {
        //             radius: 1.0,
        //             subdivisions: 12,
        //         }))
        //         .into(),
        //     transform: Transform::default().with_scale(Vec3::splat(12.0)),
        //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
        //     ..Default::default()
        // })
        // .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(8.0, 5.0, 0.0),
            border_radius: Some(6.0),
        })
        .insert(Velocity::default())
        .insert(CollisionLayers::all::<Layer>().with_group(Layer::Player))
        // .insert(Rotation(Quat::from_rotation_x(std::f32::consts::TAU)))
        .insert(Rotation(Quat::default()))
        .insert(Bird);
}

fn bird_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut bird: Query<(&mut Velocity, Entity), With<Bird>>,
    mut flap_events: EventWriter<FlapEvent>,
    mut game_state: ResMut<GameState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let (mut velocity, entity) = bird.single_mut();
        if velocity.linear.y < 0.0 {
            velocity.linear.y = 0.0;
        }
        velocity.linear.y += JUMP_FORCE;
        flap_events.send(FlapEvent);
        if matches!(*game_state, GameState::Waiting) {
            commands.entity(entity).insert(RigidBody::Dynamic);
            *game_state = GameState::Playing;
        }
    }
}

fn handle_game_reset(
    mut commands: Commands,
    mut game_reset_events: EventReader<GameResetEvent>,
    mut bird: Query<(&mut Transform, &mut Velocity, Entity), With<Bird>>,
) {
    if game_reset_events.iter().next().is_some() {
        let (mut transform, mut velocity, entity) = bird.single_mut();
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        velocity.linear = Vec3::default();
        velocity.angular = AxisAngle::default();
        commands.entity(entity).remove::<RigidBody>();
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

            println!("Up");
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

            println!("Down");
            let mut entity = commands.entity(entity);
            entity
                .remove::<EasingComponent<Rotation>>()
                .remove::<TiltUp>()
                .insert(ease)
                .insert(TiltDown);
        }
    }
}

// fn handle_bird_tilt_down(
//     mut commands: Commands,
//     bird: Query<(Entity, &Transform, &Velocity), (With<Bird>, Without<TiltDown>)>,
// ) {
//     if let Ok((entity, transform, velocity)) = bird.get_single() {
//         if velocity.linear.y < 0.0 {
//             let mut target = *transform;
//             target.rotation = Quat::from_rotation_x(0.4);
//             let ease = transform.rotation.ease_to();

//             let mut entity = commands.entity(entity);
//             entity
//                 .remove::<EasingComponent<Transform>>()
//                 .remove::<TiltUp>()
//                 .insert(ease)
//                 .insert(TiltDown);
//         }
//     }
// }
