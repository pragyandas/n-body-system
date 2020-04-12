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

  pub fn distance_between (&self, coordinate: Self) -> f32 {
    ((self.x - coordinate.x).powi(2) + (self.y - coordinate.y).powi(2)).sqrt()
  }

  // w.r.t x-axis
  pub fn angle_between (&self, coordinate: Self) -> f32 {
    ((-coordinate.y - (-self.y)) / (coordinate.x - self.x)).atan()
  }
}