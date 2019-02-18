use amethyst::{
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Velocity, Gravity};

pub struct GravitySystem;

const GRAVITY_VELOCITY: f32 = -50.;

impl <'s> System<'s> for GravitySystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Gravity>,
    );

    fn run(&mut self, (mut velocities, gravities): Self::SystemData) {
        for (velocity, _) in (&mut velocities, &gravities).join() {
            velocity.set_y(GRAVITY_VELOCITY);
        }
    }
}
