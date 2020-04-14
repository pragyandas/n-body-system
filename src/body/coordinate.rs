#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
  x: f32,
  y: f32,
}

impl Coordinate {
  pub fn new(x: f32, y: f32) -> Self {
    Coordinate {x, y}
  }

  pub fn get_x (&self) -> f32 {
    self.x
  }

  pub fn get_y (&self) -> f32 {
    self.y
  }

  pub fn get_distance_between (&self, coordinate: Self) -> f32 {
    ((self.x - coordinate.x).powi(2) + (self.y - coordinate.y).powi(2)).sqrt()
  }

  // w.r.t x-axis
  pub fn get_angle_between (&self, coordinate: Self) -> f32 {
    ((self.y - coordinate.y) / (self.x - coordinate.x)).atan()
  }

  pub fn get_direction(&self, coordinate: Self) -> (f32, f32) {
    let (mut x, mut y) = (1.0, 1.0);
    if self.x - coordinate.x < 0.0 {
      x = -1.0;
    }

    if self.y - coordinate.y < 0.0 {
      y = -1.0;
    }

    (x, y)
  }
}