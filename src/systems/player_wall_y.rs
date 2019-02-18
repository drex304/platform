use std::f32;
use std::cmp::Ordering;
use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::platform::{Player, Wall};
use crate::components::Velocity;

pub struct PlayerWallYSystem;

impl <'s> System<'s> for PlayerWallYSystem {
    type SystemData = (
        (
            WriteStorage<'s, Player>,
            ReadStorage<'s, Wall>,
            WriteStorage<'s, Transform>,
            ReadStorage<'s, Velocity>,
        )
    );

    fn run(&mut self, (mut players, walls, mut transforms, velocities): Self::SystemData) {
        for (wall, wall_transform) in (&walls, &transforms).join() {
            let (wall_x, wall_y) = (
                wall_transform.translation().x,
                wall_transform.translation().y,
            );
            for (player, player_transform, velocity) in (&mut players, &transforms, &velocities).join() {
                let (player_x, player_y) = (
                    player_transform.translation().x,
                    player_transform.translation().y,
                );
                // determine whether there has been a collision
                // bb short for binding box
                let (bb_left, bb_bottom, bb_right, bb_top) = (
                    wall_x - wall.width * 0.5 - player.width  * 0.5,
                    wall_y - wall.height * 0.5 - player.height * 0.5,
                    wall_x + wall.width * 0.5 + player.width  * 0.5,
                    wall_y + wall.height * 0.5 + player.height * 0.5,
                );
                let is_collision = point_in_rect(
                    player_x,
                    player_y,
                    bb_left,
                    bb_bottom,
                    bb_right,
                    bb_top,
                );
                if is_collision {
                    // determine direction of player movement
                    // 1 = up, -1 = down, 0 = stationary
                    player.correction_y = match velocity.get_y().partial_cmp(&0.0) {
                       Some(Ordering::Greater) => bb_bottom - player_y,
                       Some(Ordering::Less) => bb_top - player_y,
                       _ => 0.0,
                    };
                }
            }
        }
        for (player, player_transform) in (&mut players, &mut transforms).join() {
            player_transform.translate_y(player.correction_y);
            player.correction_y = 0.0;
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
