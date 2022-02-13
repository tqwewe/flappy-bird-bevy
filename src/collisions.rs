use bevy::prelude::*;
use heron::prelude::*;

use crate::{GameResetEvent, IncreaseScoreEvent, Layer};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_collisions);
    }
}

fn check_collisions(
    mut events: EventReader<CollisionEvent>,
    mut increase_score_events: EventWriter<IncreaseScoreEvent>,
    mut game_reset_events: EventWriter<GameResetEvent>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(c1, c2) => {
                if (c1.collision_layers().contains_group(Layer::Player)
                    && c2.collision_layers().contains_group(Layer::World))
                    || (c2.collision_layers().contains_group(Layer::Player)
                        && c1.collision_layers().contains_group(Layer::World))
                {
                    game_reset_events.send(GameResetEvent);
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
