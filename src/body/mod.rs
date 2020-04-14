use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub mod coordinate;

use crate::vector::Vector;
use coordinate::Coordinate;
use std::time::Instant;

use crate::system::G;

#[derive(Debug, Clone, Copy)]
pub struct Body {
  id: Instant,
  mass: f32,
  coordinate: Coordinate,
  velocity: Vector,
}

impl Body {
  pub fn new(mass: f32, coordinate: Coordinate) -> Self {
    Body {
      id: Instant::now(),
      mass,
      coordinate,
      velocity: Vector::new(0.0, 0.0),
    }
  }

  pub fn get_id(&self) -> &Instant {
    &self.id
  }

  pub fn get_coordinate(&self) -> &Coordinate {
    &self.coordinate
  }

  pub fn get_mass(&self) -> f32 {
    self.mass
  }

  pub fn add_body(&self, body: &Body) -> Self {
    let total_mass = self.mass + body.mass;
    let x = (self.get_coordinate().get_x() * self.mass + body.get_coordinate().get_x() * body.mass)
      / total_mass;
    let y = (self.get_coordinate().get_y() * self.mass + body.get_coordinate().get_y() * body.mass)
      / total_mass;

    Body::new(total_mass, Coordinate::new(x, y))
  }

  pub fn get_distance_between(&self, body: &Body) -> f32 {
    self.coordinate.get_distance_between(body.coordinate)
  }

  pub fn calculate_force_on(&self, body: &Body) -> Vector {
    let distance = self.get_distance_between(body);
    let angle = self.coordinate.get_angle_between(body.coordinate);
    let magnitude = (G * self.mass * body.mass) / distance.powi(2);
    let (x_direction, y_direction) = self.coordinate.get_direction(body.coordinate);

    Vector::new_with(magnitude, angle, Vector::new(x_direction, y_direction))
  }

  pub fn update_velocity(&mut self, acceleration: Vector, time: f32) {
    self.velocity = Vector::new(
      self.velocity.get_x() + acceleration.get_x() * time,
      self.velocity.get_y() + acceleration.get_y() * time,
    );
  }

  pub fn update_coordinate(&mut self, acceleration: Vector, time: f32) -> Vector {
    let delta = Vector::new(
      (self.velocity.get_x() * time) + (0.5 * acceleration.get_x() * time.powi(2)),
      (self.velocity.get_y() * time) + (0.5 * acceleration.get_y() * time.powi(2))
    );

    self.coordinate = Coordinate::new(
      self.coordinate.get_x() + delta.get_x(),
      self.coordinate.get_y() + delta.get_y()
    );

    delta
  }
}

impl Component for Body {
  type Storage = DenseVecStorage<Self>;
}

#[cfg(test)]
mod tests {
  use super::{Body, Coordinate};
  #[test]
  fn test_add_body() {
    let body_a = Body::new(10.0, Coordinate::new(100.0, 100.0));
    let body_b = Body::new(10.0, Coordinate::new(200.0, 200.0));

    let resultant_body = body_a.add_body(&body_b);

    assert_eq!(resultant_body.mass, 20.0);
    assert_eq!(resultant_body.coordinate.get_x(), 150.0);
    assert_eq!(resultant_body.coordinate.get_y(), 150.0);
  }
}
