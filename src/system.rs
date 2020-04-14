use amethyst::{
  assets::{AssetStorage, Loader, Handle},
  core::{transform::Transform, math::Vector3},
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use rand::{thread_rng, Rng};
use crate::body::{Body, coordinate::Coordinate};
use crate::quad_tree::QuadTree;

pub const NUM_BODIES: u32 = 1000;
pub const MAX_MASS: f32 = 100.0;
pub const ARENA_LENGTH: f32 = 1000.0;
pub const PADDING: f32 = 10.0;
// Actual value of G is 6.67e-11f64 but it's been adjusted to suit the system
pub const G: f32 = 6.67e-3f32;

pub struct SystemState;

impl SimpleState for SystemState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    world.register::<Body>();
    world.register::<QuadTree>();

    initialise_camera(world);
    initialise_quad_tree(world);
    let sprite_sheet_handle = load_sprite_sheet(world);
    initialise_bodies(world, sprite_sheet_handle.clone());
  }
}

fn initialise_camera(world: &mut World) {
  // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
  let mut transform = Transform::default();
  transform.set_translation_xyz(ARENA_LENGTH * 0.5, ARENA_LENGTH * 0.5, 1.0);

  world
      .create_entity()
      .with(Camera::standard_2d(ARENA_LENGTH, ARENA_LENGTH))
      .with(transform)
      .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  // Load the sprite sheet necessary to render the graphics.
  // The texture is the pixel data
  // `texture_handle` is a cloneable reference to the texture
  let texture_handle = {
      let loader = world.read_resource::<Loader>();
      let texture_storage = world.read_resource::<AssetStorage<Texture>>();
      loader.load(
          "texture/particle.png",
          ImageFormat::default(),
          (),
          &texture_storage,
      )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
      "texture/spritesheet.ron", // Here we load the associated ron file
      SpriteSheetFormat(texture_handle),
      (),
      &sprite_sheet_store,
  )
}

fn initialise_bodies(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet.clone(),
    sprite_number: 0
  };

  let mut rng = thread_rng();

  (1..=NUM_BODIES).into_iter().for_each(|_| {
    let x = rng.gen_range((ARENA_LENGTH / 2.0) - 10.0, (ARENA_LENGTH / 2.0) + 10.0) as f32;
    let y = rng.gen_range((ARENA_LENGTH / 2.0) - 10.0, (ARENA_LENGTH / 2.0) + 10.0) as f32;
    let mass = rng.gen_range(1.0, MAX_MASS) as f32;

    let mut local_tranform = Transform::default();
    local_tranform.set_translation_xyz(x, y, 0.0);
    let scale_factor = mass / (MAX_MASS * 0.5);
    local_tranform.set_scale(Vector3::new(scale_factor, scale_factor, 0.0));

    let body = Body::new(mass, Coordinate::new(x, y ));

    world
      .create_entity()
      .with(sprite_render.clone())
      .with(body)
      .with(local_tranform)
      .build();
  });
}

fn initialise_quad_tree(world: &mut World) {
  let quad_tree = QuadTree::new(ARENA_LENGTH - PADDING);

  world
    .create_entity()
    .with(quad_tree)
    .build();
}