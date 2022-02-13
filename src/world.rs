use bevy::prelude::*;
use heron::prelude::*;

use crate::{Layer, WIN_HEIGHT, WIN_WIDTH};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ground);
    }
}

fn setup_ground(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.02, 0.674, 0.188),
                custom_size: Some(Vec2::new(WIN_WIDTH, 20.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -(WIN_HEIGHT / 2.0) + 2.0, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(WIN_WIDTH / 2.0, 10.0, 0.0),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::World)
                .with_mask(Layer::Player),
        );

    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, WIN_HEIGHT / 2.0, 0.0),
            GlobalTransform::default(),
        ))
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(WIN_WIDTH / 2.0, 10.0, 0.0),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::World)
                .with_mask(Layer::Player),
        );
}
