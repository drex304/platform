extern crate amethyst;

mod platform;
mod systems;
mod components;

use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage
};
use amethyst::core::transform::TransformBundle;
use amethyst::utils::application_root_dir;
use amethyst::input::InputBundle;

use crate::platform::Platform;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_path = format!("{}/resources/display_config.ron", application_root_dir());
    let display_config = DisplayConfig::load(&display_path);

    let bindings_path = format!("{}/resources/binding_config.ron", application_root_dir());
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(bindings_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.3, 0.3, 0.3, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PlayerMoveSystem, "player_move_system", &["input_system"])
        .with(systems::GravitySystem, "gravity_system", &[])
        .with(systems::VelocityXSystem, "velocity_x_system", &["player_move_system", "gravity_system"])
        .with(systems::PlayerWallXSystem, "player_wall_x_system", &["velocity_x_system"])
        .with(systems::VelocityYSystem, "velocity_y_system", &["player_move_system", "gravity_system"])
        .with(systems::PlayerWallYSystem, "player_wall_y_system", &["velocity_y_system"]);
    let mut game = Application::new("./", Platform, game_data)?;

    game.run();

    Ok(())
}
