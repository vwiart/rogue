mod rogue;

extern crate amethyst;

use amethyst::utils::application_root_dir;
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig,
    DrawFlat2D,
    Pipeline,
    RenderBundle,
    Stage,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();
    let path = format!("{}/resources/display_config.ron", app_root);

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
        )?;

    use crate::rogue::Rogue;

    let mut game = Application::new("./", Rogue, game_data)?;

    game.run();

    Ok(())
}
