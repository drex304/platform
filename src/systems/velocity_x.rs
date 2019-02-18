use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::{Join, ReadStorage, Read, System, WriteStorage},
};

use crate::components::Velocity;

pub struct VelocityXSystem;

impl <'s> System<'s> for VelocityXSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        for (transform, velocities) in (&mut transforms, &velocities).join() {
            transform.translate_x(velocities.get_x() * time.delta_seconds());
        }
    }
}
