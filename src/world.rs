use bevy::prelude::*;
use heron::prelude::*;

use crate::{bird::BirdStage, GameResetEvent, GameState, Layer, WIN_HEIGHT, WIN_WIDTH};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ground)
            .add_system(handle_game_reset);
        // .add_system(handle_game_state_change.label(WorldStage).before(BirdStage));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub struct WorldStage;

fn handle_game_reset(
    mut game_reset_events: EventReader<GameResetEvent>,
    mut game_state: ResMut<GameState>,
) {
    if game_reset_events.iter().next().is_some() {
        *game_state = GameState::Waiting;
    }
}

// fn handle_game_state_change(game_state: Res<GameState>, mut gravity: ResMut<Gravity>) {
//     if game_state.is_changed() {
//         match *game_state {
//             GameState::Waiting => {}
//             GameState::Playing => {
//                 *gravity = Gravity::from(Vec3::new(0.0, -600.0, 0.0));
//             }
//         }
//     }
// }

fn setup_ground(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.02, 0.674, 0.188), // 6, 173, 48
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
