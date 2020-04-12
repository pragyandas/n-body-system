use amethyst::{
  derive::SystemDesc,
  ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::body::Body;
use crate::quad_tree::QuadTree;

#[derive(SystemDesc)]
pub struct BodyQuadTreeSystem;

impl<'s> System<'s> for BodyQuadTreeSystem {
  type SystemData = (
    WriteStorage<'s, QuadTree>,
    ReadStorage<'s, Body>,
  );

  fn run(&mut self, (mut quad_trees, bodies): Self::SystemData) {
    for quad_tree in (&mut quad_trees).join() {
      quad_tree.reset_root();

      for body in (&bodies).join() {
        quad_tree.insert(*body);
      }
    }
  }
}
