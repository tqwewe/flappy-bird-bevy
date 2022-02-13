#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bird::BirdPlugin;
use camera::CameraPlugin;
use collisions::CollisionsPlugin;
use game_state::GameStatePlugin;
use heron::prelude::*;
use pipes::PipesPlugin;
use score::ScorePlugin;
use sounds::SoundsPlugin;
use world::WorldPlugin;

mod bird;
mod camera;
mod collisions;
mod game_state;
mod pipes;
mod score;
mod sounds;
mod world;

const WIN_WIDTH: f32 = 1200.0;
const WIN_HEIGHT: f32 = 600.0;

#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
    PipeGap,
}

struct DiedEvent;
struct FlapEvent;
struct IncreaseScoreEvent;
struct GameResetEvent;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Flappy Burd".to_string(),
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.658, 0.8, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(PipesPlugin)
        .add_plugin(BirdPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(CollisionsPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(SoundsPlugin)
        .add_plugin(GameStatePlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_event::<DiedEvent>()
        .add_event::<FlapEvent>()
        .add_event::<IncreaseScoreEvent>()
        .add_event::<GameResetEvent>()
        .insert_resource(Gravity::from(Vec3::new(0.0, -600.0, 0.0)))
        .run();
}
