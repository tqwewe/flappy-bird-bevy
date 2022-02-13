use bevy::prelude::*;

const GRAVITY: f32 = 25.0;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_physic_ents)
            .add_system(update_gravity_ents);
    }
}

#[derive(Component, Default)]
pub struct PhysicsEnt {
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

impl PhysicsEnt {
    pub fn apply_force(&mut self, force: Vec3, delta: f32) {
        self.acceleration += force * delta;
    }

    pub fn reset_velocity(&mut self) {
        self.velocity = Vec3::ZERO;
    }
}

fn update_physic_ents(mut ents: Query<(&mut Transform, &mut PhysicsEnt)>) {
    for (mut transform, mut physics_ent) in ents.iter_mut() {
        let acceleration = physics_ent.acceleration;
        physics_ent.velocity += acceleration;
        transform.translation += physics_ent.velocity;
        physics_ent.acceleration = Vec3::ZERO;
    }
}

#[derive(Component)]
pub struct GravityEnt;

fn update_gravity_ents(time: Res<Time>, mut ents: Query<&mut PhysicsEnt, With<GravityEnt>>) {
    for mut physics_ent in ents.iter_mut() {
        physics_ent.apply_force(Vec3::new(0.0, -GRAVITY, 0.0), time.delta_seconds());
    }
}
