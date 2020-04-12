use amethyst::{
  core::{timing::Time, transform::Transform, SystemDesc},
  derive::SystemDesc,
  ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::body::Body;
use crate::quad_tree::QuadTree;
use crate::vector::Vector;

pub const THETA: f32 = 0.5;

#[derive(SystemDesc)]
pub struct BodyForceSystem;

impl<'s> System<'s> for BodyForceSystem {
  type SystemData = (
    ReadStorage<'s, QuadTree>,
    WriteStorage<'s, Body>,
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
  );

  fn run(&mut self, (quad_trees, mut bodies, mut transforms, time): Self::SystemData) {
    for quad_tree in (&quad_trees).join() {

    for (body, transform) in (&mut bodies, &mut transforms).join() {
        let force = quad_tree.calculate_net_force_on(&body, THETA);
        let mass = body.get_mass();
        let accelaration = Vector::new(force.get_x()/mass, force.get_y()/mass);
        let delta = body.update_coordinate(accelaration, time.delta_seconds());
        body.update_velocity(accelaration, time.delta_seconds());

        transform.prepend_translation_x(delta.get_x());
        transform.prepend_translation_y(delta.get_y());
      }
    }
  }
}

