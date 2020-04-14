use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
  x: f32,
  y: f32,
}

impl Vector {
  pub fn new(x: f32, y: f32) -> Self {
    Vector {x, y}
  }

  pub fn new_with(magnitude: f32, angle: f32, direction: Self) -> Self {
    let x = magnitude * angle.cos() * direction.x;
    let y = magnitude * angle.sin() * direction.y;

    Vector {x, y}
  }

  pub fn get_x(&self) -> f32 {
    self.x
  }

  pub fn get_y(&self) -> f32 {
    self.y
  }
}

impl Add for Vector {
  type Output = Vector;

  fn add(self, rhs: Vector) -> Vector {
      Vector::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl PartialEq for Vector {
  fn eq(&self, other: &Self) -> bool {
      self.x == other.x && self.y == other.y
  }
}