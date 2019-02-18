use std::cmp::Ordering;
use amethyst::{
    ecs::{Join, ReadStorage, Read, System, WriteStorage},
    input::InputHandler,
};

use crate::platform::Player;
use crate::components::Velocity;

pub struct PlayerMoveSystem;

const PLAYER_SPEED: f32 = 50.0;

impl <'s> System<'s> for PlayerMoveSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut velocities, players, input): Self::SystemData) {
        for (velocity, _) in (&mut velocities, &players).join() {
            // handle the x movement of the player by setting velocity
            // based on key press
            let movement = input.axis_value("player_move");
            if let Some(mv_amount) = movement {
                let x_velocity = match mv_amount.partial_cmp(&0.0) {
                    Some(Ordering::Less) => -PLAYER_SPEED,
                    Some(Ordering::Greater) => PLAYER_SPEED,
                    _ => 0.0,
                };
                velocity.set_x(x_velocity);
            }
        }
    }
}
