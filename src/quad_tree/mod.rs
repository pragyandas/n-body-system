use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub mod node;
pub mod quadrant;

use std::mem;
use node::Node;
use quadrant::Quadrant;
use super::body::{Body, coordinate::Coordinate};
use super::vector::Vector;

#[derive(Debug)]
pub struct QuadTree {
  root: Box<Node>,
}

impl QuadTree {
  pub fn new(length: f32) -> Self {
    let quadrant = Quadrant::new(0.0, 0.0, length);
    QuadTree {
      root: Box::new(Node::new(quadrant))
    }
  }

  pub fn reset_root(&mut self) {
    self.root.reset();
  }

  pub fn new_with_quadrant(quadrant: Quadrant) -> Self {
    QuadTree {
      root: Box::new(Node::new(quadrant))
    }
  }

  pub fn get_root(&self) -> &Box<Node> {
      &self.root
  }

  pub fn insert(&mut self, body: Body) {
    if self.root.get_quadrant().contains(&body.get_coordinate()) {
      self.root.insert(body);
    }
    // else {
    //   panic!("body doesn't belong to any quadrant");
    // }
  }

  pub fn merge(&mut self, trees: &mut Vec<Self>) {
    let mut nodes = trees
      .iter_mut()
      .map(|tree| {
        let root = mem::replace(&mut tree.root, Box::new(Node::new(Quadrant::new(0.0, 0.0, 0.0))));
        root
      })
      .collect();
    self.root.merge(&mut nodes);
  }

  pub fn calculate_net_force_on(&self, body: &Body, theta: f32) -> Vector {
    self.root.calculate_net_force_on(body, theta)
  }
}

impl Component for QuadTree {
  type Storage = DenseVecStorage<Self>;
}

#[cfg(test)]
mod tests {
  use super::{Body, Coordinate, QuadTree};

  #[test]
  fn test_insert() {
    let mut quad_tree = QuadTree::new(1000.0);
    let body = Body::new(10.0, Coordinate::new(900.0, 600.0));

    quad_tree.insert(body);

    assert!(quad_tree.root.get_body().is_some());
  }

  #[test]
  #[should_panic(expected = "body doesn't belong to any quadrant")]
  fn test_insert_panic() {
    let mut quad_tree = QuadTree::new(1000.0);
    let body = Body::new(10.0, Coordinate::new(900.0, 1100.0));

    quad_tree.insert(body);
  }
}