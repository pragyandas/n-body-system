use amethyst::{
  prelude::*,
  renderer::{
      plugins::{RenderFlat2D, RenderToWindow},
      types::DefaultBackend,
      RenderingBundle,
  },
  utils::application_root_dir,
};
use amethyst::core::transform::TransformBundle;

mod quad_tree;
mod body;
mod vector;
mod system;
mod systems;

use system::SystemState;

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;
  let display_config_path = app_root.join("config").join("display.ron");

  let game_data = GameDataBuilder::default()
    .with_bundle(
      RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config_path(display_config_path)?
          .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default()),
    )?
    .with_bundle(TransformBundle::new())?
    .with(systems::BodyQuadTreeSystem, "body_quad_tree_system", &[])
    .with(systems::BodyForceSystem, "body_force_system", &["body_quad_tree_system"]);

    let assets_dir = app_root.join("assets");

    let mut world = World::new();
    let mut game = Application::new(assets_dir, SystemState, game_data)?;
    game.run();

    Ok(())
}