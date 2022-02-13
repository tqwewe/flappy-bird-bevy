use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{DiedEvent, GameResetEvent};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::Waiting)
            .add_event::<GameStartedEvent>()
            .add_system_set(
                SystemSet::new()
                    .label(GameStateLabel)
                    .with_system(handle_died_event)
                    .with_system(handle_restart_input),
            );
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub struct GameStateLabel;

pub enum GameState {
    Waiting,
    Playing,
    Dead,
}

pub struct GameStartedEvent;

fn handle_died_event(mut game_state: ResMut<GameState>, mut died_events: EventReader<DiedEvent>) {
    if died_events.iter().next().is_some() {
        *game_state = GameState::Dead;
    }
}

fn handle_restart_input(
    mut game_state: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_reset_events: EventWriter<GameResetEvent>,
    mut game_started_events: EventWriter<GameStartedEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match *game_state {
            GameState::Waiting => {
                *game_state = GameState::Playing;
                game_started_events.send(GameStartedEvent);
            }
            GameState::Playing => {}
            GameState::Dead => {
                *game_state = GameState::Waiting;
                game_reset_events.send(GameResetEvent);
            }
        }
    }
}

pub fn run_if_playing(game_state: Res<GameState>) -> ShouldRun {
    match *game_state {
        GameState::Playing => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}
