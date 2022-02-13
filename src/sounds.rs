use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioSource};

use crate::{FlapEvent, GameResetEvent, IncreaseScoreEvent, Layer};

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(prepare_audio)
            .add_system(handle_game_reset)
            .add_system(handle_flap)
            .add_system(handle_point);
    }
}

struct AudioState {
    hit: Handle<AudioSource>,
    wing: Handle<AudioSource>,
    point: Handle<AudioSource>,
}

fn prepare_audio(mut commands: Commands, asset_server: ResMut<AssetServer>, audio: Res<Audio>) {
    let hit = asset_server.load("sounds/hit.mp3");
    let wing = asset_server.load("sounds/wing.mp3");
    let point = asset_server.load("sounds/point.mp3");
    let audio_state = AudioState { hit, wing, point };
    audio.set_volume(0.3);

    commands.insert_resource(audio_state);
}

fn handle_game_reset(
    mut game_reset_events: EventReader<GameResetEvent>,
    audio: Res<Audio>,
    audio_state: ResMut<AudioState>,
) {
    if game_reset_events.iter().next().is_some() {
        audio.play(audio_state.hit.clone());
    }
}

fn handle_flap(
    mut flap_events: EventReader<FlapEvent>,
    audio: Res<Audio>,
    audio_state: ResMut<AudioState>,
) {
    if flap_events.iter().next().is_some() {
        audio.play(audio_state.wing.clone());
    }
}

fn handle_point(
    mut increase_score_events: EventReader<IncreaseScoreEvent>,
    audio: Res<Audio>,
    audio_state: ResMut<AudioState>,
) {
    if increase_score_events.iter().next().is_some() {
        audio.play(audio_state.point.clone());
    }
}
