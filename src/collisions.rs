use bevy::prelude::*;
use heron::prelude::*;

use crate::{game_state::GameState, DiedEvent, IncreaseScoreEvent, Layer};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_collisions);
    }
}

fn check_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut increase_score_events: EventWriter<IncreaseScoreEvent>,
    mut died_events: EventWriter<DiedEvent>,
    game_state: Res<GameState>,
) {
    if matches!(*game_state, GameState::Playing) {
        for event in collision_events.iter() {
            match event {
                CollisionEvent::Started(c1, c2) => {
                    if (c1.collision_layers().contains_group(Layer::Player)
                        && c2.collision_layers().contains_group(Layer::World))
                        || (c2.collision_layers().contains_group(Layer::Player)
                            && c1.collision_layers().contains_group(Layer::World))
                    {
                        died_events.send(DiedEvent);
                    }
                }
                CollisionEvent::Stopped(c1, c2) => {
                    if (c1.collision_layers().contains_group(Layer::Player)
                        && c2.collision_layers().contains_group(Layer::PipeGap))
                        || (c2.collision_layers().contains_group(Layer::Player)
                            && c1.collision_layers().contains_group(Layer::PipeGap))
                    {
                        increase_score_events.send(IncreaseScoreEvent);
                    }
                }
            }
        }
    }
}
