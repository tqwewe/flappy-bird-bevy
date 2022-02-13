use bevy::prelude::*;

use crate::{GameResetEvent, IncreaseScoreEvent};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_score)
            .add_system(handle_increase_score)
            .add_system(handle_game_reset);
    }
}

#[derive(Component, Default)]
struct Score(u32);

fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Score::default());
}

fn handle_increase_score(
    mut increase_score_events: EventReader<IncreaseScoreEvent>,
    mut score_text: Query<(&mut Text, &mut Score)>,
) {
    for _ in increase_score_events.iter() {
        for (mut text, mut score) in score_text.iter_mut() {
            score.0 += 1;
            text.sections[1].value = score.0.to_string();
        }
    }
}

fn handle_game_reset(
    mut game_reset_events: EventReader<GameResetEvent>,
    mut score_text: Query<(&mut Text, &mut Score)>,
) {
    if game_reset_events.iter().next().is_some() {
        for (mut text, mut score) in score_text.iter_mut() {
            score.0 = 0;
            text.sections[1].value = "0".to_string();
        }
    }
}
