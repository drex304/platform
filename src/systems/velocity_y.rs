use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::{Join, ReadStorage, Read, System, WriteStorage},
};

use crate::components::Velocity;

pub struct VelocityYSystem;

impl <'s> System<'s> for VelocityYSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        for (transform, velocities) in (&mut transforms, &velocities).join() {
            transform.translate_y(velocities.get_y() * time.delta_seconds());
        }
    }
}
